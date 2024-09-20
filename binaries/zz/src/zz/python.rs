use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode},
    layout::Rect,
    style::palette::tailwind::GRAY,
    symbols,
    widgets::{Block, Padding, Paragraph, Widget},
};

use uv::UV;

pub struct PythonTab {
    uv: UV,
}

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
    pub fn new() -> eyre::Result<Self> {
        Ok(Self { uv: UV::new()? })
    }

    pub fn handle_events(
        &mut self,
        event: Event,
    ) -> eyre::Result<(super::AppState, Option<super::SelectedTab>)> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return Ok((super::AppState::Quitting, None)),
                KeyCode::Right => {
                    return Ok((
                        super::AppState::Running,
                        Some(super::SelectedTab::new_rust()?),
                    ))
                }
                _ => {}
            }
        }

        Ok((super::AppState::Running, None))
    }
}
