use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget},
};

use crate::{
    color_math::generate_palette,
    color_spaces::Color as dis_color,
    error::PaletteError,
    file::{load_palette, save_palette},
};

#[derive(Debug, Clone)]
enum RetryAction {
    Startup,
    Save(Vec<dis_color>),
}

#[derive(Debug, Default)]
pub struct App {
    colors: Vec<crate::color_spaces::Color>,
    exit: bool,
    error: Option<PaletteError>,
    retry_action: Option<RetryAction>,
    selected: usize,
}

impl App {
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

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());

        if let Some(error) = &self.error {
            draw_error_popup(frame, error);
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
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
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('h') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Char('l') => {
                if self.selected + 1 < self.colors.len() {
                    self.selected += 1;
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
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

    fn retry(&mut self) {
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
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(" Palette Generator ".bold())
            .title_bottom(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]).centered())
            .border_set(border::THICK);

        let inner = block.inner(area);
        block.render(area, buf);

        if self.colors.is_empty() {
            return;
        }

        let constraints = vec![Constraint::Ratio(1, self.colors.len() as u32); self.colors.len()];

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(inner);

        for (i, (color, column_area)) in self.colors.iter().zip(columns.iter()).enumerate() {
            let selected = i == self.selected;
            render_color_column(color.clone(), *column_area, buf, selected);
        }
    }
}

fn draw_error_popup(frame: &mut Frame, error: &PaletteError) {
    let area = centered_rect(frame.area(), 60, 9);
    frame.render_widget(Clear, area);

    let text = Text::from(vec![
        Line::from("An error occurred").style(Style::default().fg(Color::Red)),
        Line::from(""),
        Line::from(error.to_string()),
        Line::from(""),
        Line::from("Press Enter or Esc to continue").style(Style::default().fg(Color::Gray)),
        Line::from("Press r to retry").style(Style::default().fg(Color::Gray)),
    ]);

    let popup = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Error "))
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(popup, area);
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height) / 2),
            Constraint::Percentage(height),
            Constraint::Percentage((100 - height) / 2),
        ])
        .split(area);
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width) / 2),
            Constraint::Percentage(width),
            Constraint::Percentage((100 - width) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}

fn render_color_column(color: dis_color, area: Rect, buf: &mut Buffer, selected: bool) {
    let mut style = Style::default()
        .fg(color.ratatui_text())
        .bg(color.ratatui_color());

    if selected {
        style = style.add_modifier(ratatui::style::Modifier::BOLD);
    }

    buf.set_style(area, style);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);

    let text = Text::from(Line::styled(color.hex_to_string(), style));

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default())
        .fg(color.ratatui_color());
    if selected {
        block = block
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
    }

    Paragraph::new(text.clone())
        .style(style)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .alignment(ratatui::layout::Alignment::Center)
        .render(chunks[1], buf);
}
