use eyre::ContextCompat;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::palette::tailwind::GRAY,
    symbols,
    widgets::{Block, Padding, Paragraph, Widget, Wrap},
};

use simple_home_dir::home_dir;
use uv::UV;

pub struct PythonTab {
    uv: UV,
    prepare_uv_install: bool,
    install_uv: bool,

    zython_installed: bool,
    prepare_zython_install: bool,
    install_zython: bool,
}

impl PythonTab {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) -> eyre::Result<()> {
        if self.install_uv {
            let rt = tokio::runtime::Runtime::new()?;

            let result: eyre::Result<()> = rt.block_on(async { self.uv.install_bin().await });
            result?;
            self.prepare_uv_install = false;
            self.install_uv = false;
        }

        if self.install_zython {
            let rt = tokio::runtime::Runtime::new()?;

            let result: eyre::Result<()> = rt.block_on(async {
                #[cfg(target_os = "windows")]
                let script = script::fetch_script("https://astral.sh/uv/install.ps1").await?;
                #[cfg(not(target_os = "windows"))]
                let script = script::fetch_script("https://astral.sh/uv/install.sh").await?;

                script::execute_script(script).await?;

                Ok(())
            });

            result?;

            self.prepare_zython_install = false;
            self.install_zython = false;
            self.zython_installed = true;
        }

        if !self.uv.installed {
            let [area] = Layout::horizontal([Constraint::Percentage(50)])
                .flex(Flex::Center)
                .areas(area);

            let [area] = Layout::vertical([Constraint::Percentage(30)])
                .flex(Flex::Center)
                .areas(area);
            if self.prepare_uv_install {
                let paragraph = Paragraph::new("Installing uv...")
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(
                        Block::bordered()
                            .border_set(symbols::border::THICK)
                            .border_style(GRAY.c700),
                    )
                    .wrap(Wrap { trim: true });

                paragraph.render(area, buf);
                self.install_uv = true;
            } else {
                let paragraph = Paragraph::new("uv is not installed. Press Enter to install it.")
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(
                        Block::bordered()
                            .border_set(symbols::border::THICK)
                            .border_style(GRAY.c700),
                    )
                    .wrap(Wrap { trim: true });

                paragraph.render(area, buf);
            }
        } else if !self.zython_installed {
            let [area] = Layout::horizontal([Constraint::Percentage(50)])
                .flex(Flex::Center)
                .areas(area);

            let [area] = Layout::vertical([Constraint::Percentage(30)])
                .flex(Flex::Center)
                .areas(area);
            if self.prepare_zython_install {
                let paragraph = Paragraph::new("Installing zython...")
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(
                        Block::bordered()
                            .border_set(symbols::border::THICK)
                            .border_style(GRAY.c700),
                    )
                    .wrap(Wrap { trim: true });

                paragraph.render(area, buf);
                self.install_zython = true;
            } else {
                let paragraph =
                    Paragraph::new("zython is not installed. Press Enter to install it.")
                        .alignment(ratatui::layout::Alignment::Center)
                        .block(
                            Block::bordered()
                                .border_set(symbols::border::THICK)
                                .border_style(GRAY.c700),
                        )
                        .wrap(Wrap { trim: true });

                paragraph.render(area, buf);
            }
        }

        Ok(())
    }
}

impl PythonTab {
    pub fn new() -> eyre::Result<Self> {
        let home = home_dir().wrap_err(eyre::Report::msg("Could not find home directory"))?;

        #[cfg(target_os = "windows")]
        let bin = home.join(".cargo").join("bin").join("zython.exe");
        #[cfg(not(target_os = "windows"))]
        let bin = home.join(".cargo").join("bin").join("zython");

        let zython_installed = bin.exists();

        Ok(Self {
            uv: UV::new()?,
            prepare_uv_install: false,
            install_uv: false,

            zython_installed,
            prepare_zython_install: false,
            install_zython: false,
        })
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
                KeyCode::Enter => {
                    if !self.uv.installed {
                        self.prepare_uv_install = true;

                        return Ok((super::AppState::Running, None));
                    } else if !self.zython_installed {
                        self.prepare_zython_install = true;

                        return Ok((super::AppState::Running, None));
                    }
                }
                _ => {}
            }
        }

        Ok((super::AppState::Running, None))
    }
}
