use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    colors: Vec<(crate::color_spaces::Color)>,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            _ => todo!(),
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Palette Generator".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q>".blue().bold()]);
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
    }
}
