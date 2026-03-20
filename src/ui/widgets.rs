use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::model::Button;

pub fn render_button(button: &Button, is_selected: bool) -> Paragraph<'_> {
    let style = if is_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let border_style = if is_selected {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    Paragraph::new(Line::from(button.label.as_str()))
        .alignment(Alignment::Center)
        .style(style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(if is_selected { "Selected" } else { "Button" }),
        )
}