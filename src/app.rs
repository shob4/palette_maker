use crate::{
    color_math::{generate_color, generate_palette, monochromatic},
    color_spaces::Color as dis_color,
    error::PaletteError,
    file::{load_palette, save_palette},
    mode::{RetryAction, UiMode},
    ui::draw_error_popup,
};
use ratatui::{DefaultTerminal, Frame};
use std::io;

#[derive(Debug, Default)]
pub struct App {
    pub colors: Vec<crate::color_spaces::Color>,
    pub exit: bool,
    pub error: Option<PaletteError>,
    pub retry_action: Option<RetryAction>,
    pub selected: usize,
    pub locked: bool,
    pub num_locked: u8,
    pub mode: UiMode,
}

impl App {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());

        if let Some(error) = &self.error {
            draw_error_popup(frame, error);
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let palette = match self.startup() {
            Ok(palette) => {
                self.colors = palette.clone();
                palette
            }
            Err(e) => {
                self.error = Some(e);
                self.retry_action = Some(RetryAction::Startup);
                Vec::new()
            }
        };
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        self.shutdown(palette);
        Ok(())
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }

    fn startup(&mut self) -> Result<Vec<dis_color>, PaletteError> {
        let start_palette = match load_palette("cache") {
            Ok(palette) => palette,
            Err(_) => generate_palette(5)?,
        };

        Ok(start_palette)
    }

    fn shutdown(&mut self, palette: Vec<dis_color>) {
        match save_palette("cache", palette.clone()) {
            Ok(()) => (),
            Err(e) => {
                self.error = Some(e);
                self.retry_action = Some(RetryAction::Save(palette));
            }
        }
    }

    pub fn retry(&mut self) {
        if let Some(action) = self.retry_action.clone() {
            self.error = None;
            self.retry_action = None;

            match action {
                RetryAction::Startup => {
                    if let Err(e) = self.startup().map(|p| self.colors = p) {
                        self.error = Some(e);
                        self.retry_action = Some(RetryAction::Startup);
                    }
                }
                RetryAction::Save(palette) => {
                    if let Err(e) = save_palette("cache", palette.clone()) {
                        self.error = Some(e);
                        self.retry_action = Some(RetryAction::Save(palette));
                    }
                }
                RetryAction::Generate(size) => {
                    if let Err(e) = generate_palette(size) {
                        self.error = Some(e);
                        self.retry_action = Some(RetryAction::Generate(size));
                    }
                }
                RetryAction::GenerateSingle => {
                    if let Err(e) = generate_color() {
                        self.error = Some(e);
                        self.retry_action = Some(RetryAction::GenerateSingle);
                    }
                }
                RetryAction::Monochrome(hsl) => {
                    if let Err(e) = monochromatic(&hsl) {
                        self.error = Some(e);
                        self.retry_action = Some(RetryAction::Monochrome(hsl));
                    }
                }
            }
        }
    }
}
