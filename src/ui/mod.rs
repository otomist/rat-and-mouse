mod widgets;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Frame,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::examples::clickable_title_footer::{FOOTER_ID, TITLE_ID};
use crate::model::AppState;

pub fn draw(frame: &mut Frame<'_>, state: &mut AppState) {
    let area = frame.area();
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(area);

    state.set_clickable_rect(TITLE_ID, sections[0]);
    let title_text = state
        .clickable_label(TITLE_ID)
        .unwrap_or("Example Ratatui App (click me)");
    let title = Paragraph::new(Line::from(title_text))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM).title("Clickable"));
    frame.render_widget(title, sections[0]);

    render_button_grid(frame, sections[1], state);

    state.set_clickable_rect(FOOTER_ID, sections[2]);
    let footer_text = state
        .clickable_label(FOOTER_ID)
        .unwrap_or("Example footer paragraph (click me)");
    let footer = Paragraph::new(Text::from(footer_text))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::TOP).title("Clickable Footer"));
    frame.render_widget(footer, sections[2]);
}

fn render_button_grid(frame: &mut Frame<'_>, area: Rect, state: &mut AppState) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(9),
            Constraint::Percentage(20),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(50),
            Constraint::Percentage(20),
        ])
        .split(vertical[1]);

    let grid = horizontal[1];
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Length(4)])
        .split(grid);

    let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Length(2), Constraint::Length(24)])
        .split(rows[0]);
    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Length(2), Constraint::Length(24)])
        .split(rows[1]);

    let button_rects = [top_row[0], top_row[2], bottom_row[0], bottom_row[2]];

    for button in &mut state.buttons {
        button.rect = Rect::default();
    }

    for (index, (button, rect)) in state
        .buttons
        .iter_mut()
        .zip(button_rects.into_iter())
        .enumerate()
    {
        button.rect = rect;
        let is_selected = index == state.selected;
        frame.render_widget(widgets::render_button(button, is_selected), rect);
    }

    let hint_area = Rect {
        x: grid.x,
        y: grid.y.saturating_add(grid.height.saturating_sub(1)),
        width: grid.width,
        height: 1,
    };

    let hint = Paragraph::new(Line::from(state.status.as_str())).alignment(Alignment::Center);
    frame.render_widget(hint, hint_area);
}