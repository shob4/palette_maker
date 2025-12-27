use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph, Widget},
};

use crate::color_spaces::Color as dis_color;
use crate::{
    color_math::generate_palette,
    file::{load_palette, save_palette},
};

#[derive(Debug, Default)]
pub struct App {
    colors: Vec<crate::color_spaces::Color>,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.startup();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        self.shutdown(palette);
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn startup(&mut self) -> Vec<dis_color> {
        let start_palette = match load_palette("cache") {
            Ok(palette) => palette,
            Err(_) => match generate_palette(5) {
                Ok(palette) => palette,
                Err(_) => todo!(),
            },
        };

        start_palette
    }

    fn shutdown(&mut self, palette: Vec<dis_color>) {
        match save_palette("cache", palette) {
            Ok(()) => (),
            Err(_) => todo!(),
        }
    }
    fn handle_error(&mut self, error: &'static str) {}
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Palette Generator ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let items: Vec<ListItem> = self
            .colors
            .iter()
            .map(|c| {
                ListItem::new(Line::from(vec![
                    Span::styled(" ", Style::default().fg(c.ratatui_text())),
                    Span::styled(c.hex_to_string(), Style::default().bg(c.ratatui_color())),
                ]))
            })
            .collect();
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        List::new(items).block(block).render(area, buf);
    }
}
