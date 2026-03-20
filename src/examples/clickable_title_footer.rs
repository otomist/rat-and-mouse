use crate::{
    app::App,
    model::{AppState, Button},
};

pub const TITLE_ID: &str = "chrome.title";
pub const FOOTER_ID: &str = "chrome.footer";

pub fn create_items() -> Vec<Button> {
    vec![
        Button::new(TITLE_ID, "Example Ratatui App (click me)"),
        Button::new(
            FOOTER_ID,
            "Example footer paragraph (click me) - this demonstrates non-button clickable items.",
        ),
    ]
}

pub fn register_handlers(app: &mut App) {
    app.register_handler(TITLE_ID.into(), |state: &mut AppState| {
        state.status = "Action: Title clicked".to_string();
    });

    app.register_handler(FOOTER_ID.into(), |state: &mut AppState| {
        state.status = "Action: Footer clicked".to_string();
    });
}