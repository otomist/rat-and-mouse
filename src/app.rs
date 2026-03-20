use std::{collections::HashMap, io, time::Duration};

use crossterm::event;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    examples::clickable_title_footer,
    input,
    model::{AppState, Button, ButtonId, UiEvent},
    ui,
};

type ActionHandler = Box<dyn FnMut(&mut AppState)>;

pub struct App {
    pub state: AppState,
    handlers: HashMap<ButtonId, ActionHandler>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            state: AppState {
                status: "Use Tab, Shift+Tab, arrow keys, or click a button. Press Enter to activate. Press q to quit.".to_string(),
                clickables: clickable_title_footer::create_items(),
                buttons: vec![
                    Button::new("button.one", "Button 1"),
                    Button::new("button.two", "Button 2"),
                    Button::new("button.three", "Button 3"),
                    Button::new("button.four", "Button 4"),
                ],
                selected: 0,
            },
            handlers: HashMap::new(),
        };

        app.register_default_handlers();
        clickable_title_footer::register_handlers(&mut app);
        app
    }

    pub fn register_handler<F>(&mut self, id: ButtonId, handler: F)
    where
        F: FnMut(&mut AppState) + 'static,
    {
        self.handlers.insert(id, Box::new(handler));
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| ui::draw(frame, &mut self.state))?;

            if event::poll(Duration::from_millis(100))? {
                let raw_event = event::read()?;
                if let Some(ui_event) = input::to_ui_event(raw_event, &self.state) {
                    if self.handle_ui_event(ui_event) {
                        return Ok(());
                    }
                }
            }
        }
    }

    fn handle_ui_event(&mut self, ui_event: UiEvent) -> bool {
        match ui_event {
            UiEvent::Quit => true,
            UiEvent::FocusNext => {
                self.state.focus_next();
                self.set_focused_status();
                false
            }
            UiEvent::FocusPrevious => {
                self.state.focus_previous();
                self.set_focused_status();
                false
            }
            UiEvent::FocusLeft => {
                self.state.focus_left();
                self.set_focused_status();
                false
            }
            UiEvent::FocusRight => {
                self.state.focus_right();
                self.set_focused_status();
                false
            }
            UiEvent::FocusUp => {
                self.state.focus_up();
                self.set_focused_status();
                false
            }
            UiEvent::FocusDown => {
                self.state.focus_down();
                self.set_focused_status();
                false
            }
            UiEvent::ActivateFocused => {
                if let Some(id) = self.state.selected_button_id() {
                    self.dispatch_click(id);
                }
                false
            }
            UiEvent::ButtonClicked(id) => {
                self.state.focus_by_id(&id);
                self.dispatch_click(id);
                false
            }
        }
    }

    fn set_focused_status(&mut self) {
        if let Some(label) = self.state.selected_label() {
            self.state.status = format!("Focused {label}");
        }
    }

    fn dispatch_click(&mut self, id: ButtonId) {
        if let Some(mut handler) = self.handlers.remove(&id) {
            handler(&mut self.state);
            self.handlers.insert(id, handler);
            return;
        }

        if let Some(label) = self.state.element_label(&id) {
            self.state.status = format!("Clicked {label} (no handler registered)");
        }
    }

    fn register_default_handlers(&mut self) {
        self.register_handler(ButtonId::from("button.one"), |state| {
            state.status = "Action: Button 1 handler executed".to_string();
        });
        self.register_handler(ButtonId::from("button.two"), |state| {
            state.status = "Action: Button 2 handler executed".to_string();
        });
        self.register_handler(ButtonId::from("button.three"), |state| {
            state.status = "Action: Button 3 handler executed".to_string();
        });
        self.register_handler(ButtonId::from("button.four"), |state| {
            state.status = "Action: Button 4 handler executed".to_string();
        });
    }
}