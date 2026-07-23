use crate::{
    app::App, color_spaces::Color as dis_color, error::PaletteError, input::TextInput, mode::UiMode,
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

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

        for (i, column_area) in columns.iter().enumerate() {
            match &self.mode {
                UiMode::Monochrome {
                    column,
                    options,
                    selected,
                } if *column == i => {
                    render_monochrome_column(options, *selected, *column_area, buf);
                }
                _ => {
                    let color = self.colors[i].clone();
                    let selected = i == self.selected;
                    render_color_column(color, *column_area, buf, selected);
                }
            }
        }
    }
}

pub fn draw_error_popup(frame: &mut Frame, error: &PaletteError) {
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

pub fn draw_save_popup(frame: &mut Frame, input: &TextInput) {
    let area = centered_rect(frame.area(), 60, 9);
    frame.render_widget(Clear, area);

    let block = Block::default().borders(Borders::ALL).title(" Save as ");
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(input, inner);

    frame.set_cursor_position((inner.x + input.cursor_col() + 1, inner.y));
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
        .constraints([Constraint::Min(4), Constraint::Length(4)])
        .split(area);

    let mut text = Text::from(vec![
        Line::styled("", style),
        Line::styled(color.hex_to_string(), style),
    ]);
    if color.locked {
        text = Text::from(vec![
            Line::styled("", style),
            Line::styled(color.hex_to_string(), style),
        ]);
    }

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

fn render_monochrome_column(colors: &[dis_color], selected: usize, area: Rect, buf: &mut Buffer) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Ratio(1, colors.len() as u32);
            colors.len()
        ])
        .split(area);

    for (i, (color, row)) in colors.iter().zip(rows.iter()).enumerate() {
        let base_style = Style::default()
            .bg(color.ratatui_color())
            .fg(color.ratatui_text());

        let block = if i == selected {
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default())
                .fg(color.ratatui_text())
        } else {
            Block::default()
        };

        let text = Line::from(Span::styled(color.hex_to_string(), base_style));

        Paragraph::new(text)
            .block(block)
            .style(base_style)
            .alignment(ratatui::layout::Alignment::Center)
            .render(*row, buf);
    }
}
