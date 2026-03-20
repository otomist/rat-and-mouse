use ratatui::layout::Rect;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ButtonId(String);

impl ButtonId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ButtonId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ButtonId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug)]
pub struct Button {
    pub id: ButtonId,
    pub label: String,
    pub rect: Rect,
    pub enabled: bool,
}

impl Button {
    pub fn new(id: impl Into<ButtonId>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            rect: Rect::default(),
            enabled: true,
        }
    }

    pub fn hit_test(&self, x: u16, y: u16) -> bool {
        x >= self.rect.x
            && x < self.rect.x.saturating_add(self.rect.width)
            && y >= self.rect.y
            && y < self.rect.y.saturating_add(self.rect.height)
    }
}

#[derive(Debug)]
pub struct AppState {
    pub status: String,
    pub clickables: Vec<Button>,
    pub buttons: Vec<Button>,
    pub selected: usize,
}

impl AppState {
    pub fn selected_button_id(&self) -> Option<ButtonId> {
        self.buttons.get(self.selected).map(|button| button.id.clone())
    }

    pub fn selected_label(&self) -> Option<&str> {
        self.buttons.get(self.selected).map(|button| button.label.as_str())
    }

    pub fn element_label(&self, id: &ButtonId) -> Option<&str> {
        if let Some(label) = self
            .clickables
            .iter()
            .find(|item| &item.id == id)
            .map(|item| item.label.as_str())
        {
            return Some(label);
        }

        self.buttons
            .iter()
            .find(|button| &button.id == id)
            .map(|button| button.label.as_str())
    }

    pub fn clickable_label(&self, id: impl Into<ButtonId>) -> Option<&str> {
        let id = id.into();
        self.clickables
            .iter()
            .find(|item| item.id == id)
            .map(|item| item.label.as_str())
    }

    pub fn set_clickable_rect(&mut self, id: impl Into<ButtonId>, rect: Rect) {
        let id = id.into();
        if let Some(item) = self.clickables.iter_mut().find(|item| item.id == id) {
            item.rect = rect;
        }
    }

    pub fn focus_by_id(&mut self, id: &ButtonId) {
        if let Some(index) = self.buttons.iter().position(|button| &button.id == id) {
            self.selected = index;
        }
    }

    pub fn focus_next(&mut self) {
        if self.buttons.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.buttons.len();
    }

    pub fn focus_previous(&mut self) {
        if self.buttons.is_empty() {
            return;
        }
        self.selected = (self.selected + self.buttons.len() - 1) % self.buttons.len();
    }

    pub fn focus_left(&mut self) {
        if self.selected % 2 == 1 {
            self.selected -= 1;
        }
    }

    pub fn focus_right(&mut self) {
        if self.selected % 2 == 0 && self.selected + 1 < self.buttons.len() {
            self.selected += 1;
        }
    }

    pub fn focus_up(&mut self) {
        if self.selected >= 2 {
            self.selected -= 2;
        }
    }

    pub fn focus_down(&mut self) {
        if self.selected + 2 < self.buttons.len() {
            self.selected += 2;
        }
    }

    pub fn clicked_button_id_at(&self, x: u16, y: u16) -> Option<ButtonId> {
        if let Some(id) = self
            .clickables
            .iter()
            .find(|item| item.enabled && item.hit_test(x, y))
            .map(|item| item.id.clone())
        {
            return Some(id);
        }

        self.buttons
            .iter()
            .find(|button| button.enabled && button.hit_test(x, y))
            .map(|button| button.id.clone())
    }
}

#[derive(Clone, Debug)]
pub enum UiEvent {
    Quit,
    FocusNext,
    FocusPrevious,
    FocusLeft,
    FocusRight,
    FocusUp,
    FocusDown,
    ActivateFocused,
    ButtonClicked(ButtonId),
}