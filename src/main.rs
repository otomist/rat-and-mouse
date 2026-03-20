use std::io;
use std::time::Duration;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
        MouseButton, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

const BUTTON_LABELS: [&str; 4] = ["Button 1", "Button 2", "Button 3", "Button 4"];

fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;
    let app_result = App::default().run(&mut terminal);
    restore_terminal(&mut terminal)?;
    app_result
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()
}

#[derive(Debug)]
struct App {
    selected: usize,
    status: String,
    button_areas: [Rect; 4],
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected: 0,
            status: String::from("Use Tab, Shift+Tab, arrow keys, or click a button. Press Enter to activate. Press q to quit."),
            button_areas: [Rect::default(); 4],
        }
    }
}

impl App {
    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) if self.handle_key(key) => return Ok(()),
                    Event::Mouse(mouse) => self.handle_mouse(mouse),
                    Event::Resize(_, _) => {}
                    _ => {}
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(5),
            ])
            .split(area);

        let title = Paragraph::new(Line::from("Example Ratatui App"))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(title, sections[0]);

        self.render_buttons(frame, sections[1]);

        let footer = Paragraph::new(Text::from(
            "This is an example paragraph at the bottom of the screen. It stays centered in the layout while the button grid remains interactive in the middle.",
        ))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::TOP).title("Example Paragraph"));
        frame.render_widget(footer, sections[2]);
    }

    fn render_buttons(&mut self, frame: &mut Frame, area: Rect) {
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
            .margin(0)
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
        self.button_areas = button_rects;

        for (index, rect) in button_rects.into_iter().enumerate() {
            let is_selected = index == self.selected;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let label = Paragraph::new(Line::from(BUTTON_LABELS[index]))
                .alignment(Alignment::Center)
                .style(style)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(if is_selected {
                            Style::default().fg(Color::Cyan)
                        } else {
                            Style::default().fg(Color::DarkGray)
                        })
                        .title(if is_selected { "Selected" } else { "Button" }),
                );
            frame.render_widget(label, rect);
        }

        let hint_area = Rect {
            x: grid.x,
            y: grid.y.saturating_add(grid.height.saturating_sub(1)),
            width: grid.width,
            height: 1,
        };
        let hint = Paragraph::new(Line::from(self.status.as_str())).alignment(Alignment::Center);
        frame.render_widget(hint, hint_area);
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => true,
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.previous_button();
                } else {
                    self.next_button();
                }
                false
            }
            KeyCode::Right => {
                if self.selected % 2 == 0 {
                    self.selected += 1;
                }
                false
            }
            KeyCode::Left => {
                if self.selected % 2 == 1 {
                    self.selected -= 1;
                }
                false
            }
            KeyCode::Down => {
                if self.selected < 2 {
                    self.selected += 2;
                }
                false
            }
            KeyCode::Up => {
                if self.selected >= 2 {
                    self.selected -= 2;
                }
                false
            }
            KeyCode::Enter => {
                self.activate_selected();
                false
            }
            _ => false,
        }
    }

    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
            for (index, area) in self.button_areas.iter().enumerate() {
                if contains_point(*area, mouse.column, mouse.row) {
                    self.selected = index;
                    self.activate_selected();
                    break;
                }
            }
        }
    }

    fn next_button(&mut self) {
        self.selected = (self.selected + 1) % BUTTON_LABELS.len();
        self.status = format!("Focused {}", BUTTON_LABELS[self.selected]);
    }

    fn previous_button(&mut self) {
        self.selected = (self.selected + BUTTON_LABELS.len() - 1) % BUTTON_LABELS.len();
        self.status = format!("Focused {}", BUTTON_LABELS[self.selected]);
    }

    fn activate_selected(&mut self) {
        self.status = format!("Activated {}", BUTTON_LABELS[self.selected]);
    }
}

fn contains_point(area: Rect, x: u16, y: u16) -> bool {
    x >= area.x
        && x < area.x.saturating_add(area.width)
        && y >= area.y
        && y < area.y.saturating_add(area.height)
}
