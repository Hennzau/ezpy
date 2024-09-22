use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode},
    layout::Rect,
    style::palette::tailwind::RED,
    symbols,
    widgets::{Block, Padding, Paragraph, Widget},
};

pub struct ZedTab {}

impl ZedTab {
    pub fn new() -> eyre::Result<Self> {
        Ok(Self {})
    }

    pub fn handle_events(
        &mut self,
        event: Event,
    ) -> eyre::Result<(super::AppState, Option<super::SelectedTab>)> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return Ok((super::AppState::Quitting, None)),
                KeyCode::Left => {
                    return Ok((
                        super::AppState::Running,
                        Some(super::SelectedTab::new_cxx()?),
                    ))
                }
                _ => {}
            }
        }

        Ok((super::AppState::Running, None))
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) -> eyre::Result<()> {
        Paragraph::new("Not available yet.")
            .block(
                Block::bordered()
                    .border_set(symbols::border::PROPORTIONAL_TALL)
                    .padding(Padding::horizontal(1))
                    .border_style(RED.c700),
            )
            .render(area, buf);

        Ok(())
    }
}
