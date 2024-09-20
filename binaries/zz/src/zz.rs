use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    text::Line,
    widgets::{Tabs, Widget},
    DefaultTerminal,
};

pub mod cxx;
pub mod python;
pub mod rust;
pub mod zed;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Running,
    Quitting,
}

pub enum SelectedTab {
    Python(python::PythonTab),
    Rust(rust::RustTab),
    CXX(cxx::CXXTab),
    Zed(zed::ZedTab),
}

impl SelectedTab {
    pub fn new_python() -> Self {
        Self::Python(python::PythonTab::new())
    }

    pub fn new_rust() -> Self {
        Self::Rust(rust::RustTab::new())
    }

    pub fn new_cxx() -> Self {
        Self::CXX(cxx::CXXTab::new())
    }

    pub fn new_zed() -> Self {
        Self::Zed(zed::ZedTab::new())
    }

    fn palette(&self) -> tailwind::Palette {
        match self {
            SelectedTab::Python(_) => tailwind::GRAY,
            SelectedTab::Rust(_) => tailwind::RED,
            SelectedTab::CXX(_) => tailwind::RED,
            SelectedTab::Zed(_) => tailwind::RED,
        }
    }

    fn handle_events(&mut self, event: Event) -> (AppState, Option<Self>) {
        match self {
            SelectedTab::Python(tab) => tab.handle_events(event),
            SelectedTab::Rust(tab) => tab.handle_events(event),
            SelectedTab::CXX(tab) => tab.handle_events(event),
            SelectedTab::Zed(tab) => tab.handle_events(event),
        }
    }
}

impl Widget for &SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            SelectedTab::Python(tab) => tab.render(area, buf),
            SelectedTab::Rust(tab) => tab.render(area, buf),
            SelectedTab::CXX(tab) => tab.render(area, buf),
            SelectedTab::Zed(tab) => tab.render(area, buf),
        }
    }
}

pub struct App {
    pub state: AppState,
    pub selected_tab: SelectedTab,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> eyre::Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> eyre::Result<()> {
        let event = event::read()?;

        let (state, tab): (AppState, Option<SelectedTab>) = self.selected_tab.handle_events(event);

        if let Some(tab) = tab {
            self.selected_tab = tab;
        }

        self.state = state;

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        /* header */
        "ZZ: your dev tools".bold().render(title_area, buf);

        /* tabs title */

        let titles = [
            "   Python   "
                .fg(tailwind::SLATE.c200)
                .bg(tailwind::GRAY.c900),
            "   Rust   ".fg(tailwind::SLATE.c200).bg(tailwind::RED.c900),
            "   C++   ".fg(tailwind::SLATE.c200).bg(tailwind::RED.c900),
            "   Zed   ".fg(tailwind::SLATE.c200).bg(tailwind::RED.c900),
        ];

        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = match self.selected_tab {
            SelectedTab::Python(_) => 0,
            SelectedTab::Rust(_) => 1,
            SelectedTab::CXX(_) => 2,
            SelectedTab::Zed(_) => 3,
        };

        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(tabs_area, buf);

        /* render content */

        self.selected_tab.render(inner_area, buf);

        /* footer */

        Line::raw("Use arrow keys to navigate | Press ESC to quit")
            .centered()
            .render(footer_area, buf);
    }
}
