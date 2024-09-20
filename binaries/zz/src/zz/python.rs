use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{self, GRAY},
        Color, Stylize,
    },
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
    DefaultTerminal,
};

pub struct PythonTab {}

impl Widget for &PythonTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Python python python")
            .block(
                Block::bordered()
                    .border_set(symbols::border::PROPORTIONAL_TALL)
                    .padding(Padding::horizontal(1))
                    .border_style(GRAY.c700),
            )
            .render(area, buf);
    }
}

impl PythonTab {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_events(&mut self, event: Event) -> (super::AppState, Option<super::SelectedTab>) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return (super::AppState::Quitting, None),
                KeyCode::Right => {
                    return (
                        super::AppState::Running,
                        Some(super::SelectedTab::new_rust()),
                    )
                }
                _ => {}
            }
        }

        (super::AppState::Running, None)
    }
}
