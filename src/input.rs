use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

use crate::model::{AppState, UiEvent};

pub fn to_ui_event(event: Event, state: &AppState) -> Option<UiEvent> {
    match event {
        Event::Key(key) => key_to_ui_event(key),
        Event::Mouse(mouse) => mouse_to_ui_event(mouse, state),
        Event::Resize(_, _) => None,
        Event::FocusGained | Event::FocusLost | Event::Paste(_) => None,
    }
}

fn key_to_ui_event(key: KeyEvent) -> Option<UiEvent> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Some(UiEvent::Quit),
        KeyCode::Tab if key.modifiers.contains(KeyModifiers::SHIFT) => {
            Some(UiEvent::FocusPrevious)
        }
        KeyCode::Tab => Some(UiEvent::FocusNext),
        KeyCode::Left => Some(UiEvent::FocusLeft),
        KeyCode::Right => Some(UiEvent::FocusRight),
        KeyCode::Up => Some(UiEvent::FocusUp),
        KeyCode::Down => Some(UiEvent::FocusDown),
        KeyCode::Enter => Some(UiEvent::ActivateFocused),
        _ => None,
    }
}

fn mouse_to_ui_event(mouse: MouseEvent, state: &AppState) -> Option<UiEvent> {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => state
            .clicked_button_id_at(mouse.column, mouse.row)
            .map(UiEvent::ButtonClicked),
        _ => None,
    }
}