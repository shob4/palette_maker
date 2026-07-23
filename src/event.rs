use crate::{
    app::App,
    color_math::{generate_color, generate_palette, generate_palette_from_base, monochromatic},
    color_spaces::Color as dis_color,
    file::save_palette,
    input::TextInput,
    mode::{RetryAction, UiMode},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

impl App {
    pub fn handle_events(&mut self) -> io::Result<()> {
        match &self.mode {
            UiMode::Normal => {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        self.handle_key_event(key_event)
                    }
                    _ => {}
                };
            }
            UiMode::Monochrome {
                column: _,
                options: _,
                selected: _,
            } => match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_monochrome_key_event(key_event)
                }
                _ => {}
            },
            UiMode::Save { input: _ } => match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_save_event(key_event)
                }
                _ => {}
            }, // need to add save mode, edit mode, copy mode(?)
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.error.is_some() {
            match key_event.code {
                KeyCode::Enter | KeyCode::Esc => {
                    self.error = None;
                }
                KeyCode::Char('r') => {
                    self.retry();
                }
                _ => {}
            }
        }
        match key_event.code {
            // quit
            KeyCode::Char('q') => self.exit(),
            // move left or wrap to last
            KeyCode::Char('h') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            // move right or wrap to first
            KeyCode::Char('l') => {
                if self.selected + 1 < self.colors.len() {
                    self.selected += 1;
                }
            }
            // randomize all unlocked colors
            KeyCode::Char(' ') => {
                if self.locked {
                    let mut slots: Vec<Option<dis_color>> = self
                        .colors
                        .iter()
                        .map(|c| if c.locked { Some(c.clone()) } else { None })
                        .collect();

                    let locked_colors: Vec<dis_color> =
                        slots.iter().filter_map(|c| c.clone()).collect();

                    let mut generated = match generate_palette_from_base(
                        &locked_colors,
                        self.colors.len() - locked_colors.len(),
                    ) {
                        Ok(palette) => palette,
                        Err(e) => {
                            self.error = Some(e);
                            self.retry_action = Some(RetryAction::Generate(self.colors.len() - 1));
                            return;
                        }
                    };

                    for slot in slots.iter_mut() {
                        if slot.is_none() {
                            *slot = Some(generated.remove(0));
                        }
                    }
                    self.colors = slots.into_iter().map(Option::unwrap).collect();
                    return;
                }
                self.colors = match generate_palette(self.colors.len() - 1) {
                    Ok(palette) => palette,
                    Err(e) => {
                        self.error = Some(e);
                        self.retry_action = Some(RetryAction::Generate(self.colors.len() - 1));
                        return;
                    }
                }
            }
            // Lock a color, prevent randomization
            KeyCode::Char('L') => {
                if self.colors[self.selected].locked {
                    self.colors[self.selected].locked = false;
                    self.num_locked -= 1;
                    if self.num_locked == 0 {
                        self.locked = false;
                    }
                    return;
                }
                self.colors[self.selected].locked = true;
                self.locked = true;
                self.num_locked += 1;
            }
            // randomize highlighted color
            KeyCode::Char('r') => {
                if self.colors[self.selected].locked {
                    return;
                }
                self.colors[self.selected] = match generate_color() {
                    Ok(color) => color,
                    Err(e) => {
                        self.error = Some(e);
                        self.retry_action = Some(RetryAction::GenerateSingle);
                        return;
                    }
                };
            }
            // select from monochromatic scale of color
            KeyCode::Char('m') => {
                if matches!(self.mode, UiMode::Normal) {
                    let options = match monochromatic(&self.colors[self.selected].hsl) {
                        Ok(palette) => palette,
                        Err(e) => {
                            self.error = Some(e);
                            self.retry_action = Some(RetryAction::Monochrome(
                                self.colors[self.selected].hsl.clone(),
                            ));
                            return;
                        }
                    };

                    self.mode = UiMode::Monochrome {
                        column: self.selected,
                        options: options.clone(),
                        selected: options.len() / 2,
                    };
                }
            }
            // save the palette
            KeyCode::Char('s') => {
                self.mode = UiMode::Save {
                    input: TextInput::new(),
                };
            }
            // menu for copying selected color
            // a | 1 for all encodings,
            // h | 2 for hsl,
            // r | 3 for rgb,
            KeyCode::Char('c') => {
                return;
            }
            // edit selected color
            KeyCode::Char('e') => {
                return;
            }
            // add another color
            // does it need a click?
            KeyCode::Char('a') => {
                return;
            }
            _ => {}
        }
    }

    fn handle_monochrome_key_event(&mut self, key_event: KeyEvent) {
        if self.error.is_some() {
            match key_event.code {
                KeyCode::Enter | KeyCode::Esc => {
                    self.error = None;
                }
                KeyCode::Char('r') => {
                    self.retry();
                }
                _ => {}
            }
        }
        if let UiMode::Monochrome {
            column,
            options,
            selected,
        } = &mut self.mode
        {
            match key_event.code {
                KeyCode::Char('j') => {
                    if *selected + 1 < options.len() {
                        *selected += 1;
                    }
                }
                KeyCode::Char('k') => {
                    if *selected > 0 {
                        *selected -= 1;
                    }
                }
                KeyCode::Enter => {
                    self.colors[*column] = options[*selected].clone();
                    self.mode = UiMode::Normal;
                }
                KeyCode::Esc => {
                    self.mode = UiMode::Normal;
                }
                KeyCode::Char('q') => {
                    self.exit();
                }
                _ => {}
            }
        }
    }

    fn handle_save_event(&mut self, key_event: KeyEvent) {
        let UiMode::Save { input } = &mut self.mode else {
            return;
        };

        match key_event.code {
            KeyCode::Char(c) => input.insert_char(c),
            KeyCode::Backspace => input.delete_char_before_cursor(),
            KeyCode::Delete => input.delete_char_after_cursor(),
            KeyCode::Left => input.move_left(),
            KeyCode::Right => input.move_right(),
            KeyCode::Enter => {
                let name = input.value().to_string();
                if let Err(e) = save_palette(&name, self.colors.clone()) {
                    self.error = Some(e);
                    self.retry_action = Some(RetryAction::Save(self.colors.clone()));
                }
                self.mode = UiMode::Normal;
            }
            KeyCode::Esc => self.mode = UiMode::Normal,
            _ => {}
        }
    }
}
