use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
    DefaultTerminal,
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

fn main() -> eyre::Result<()> {
    let terminal = ratatui::init();

    let app_result = App {
        state: AppState::Running(RunningState::Main),
        selected_tab: SelectedTab::PythonTab,
    }
    .run(terminal);

    ratatui::restore();

    app_result
}

#[derive(Default)]
struct App {
    state: AppState,
    selected_tab: SelectedTab,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum RunningState {
    #[default]
    Main,
    InTab,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    Running(RunningState),

    #[default]
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Python")]
    PythonTab,
    #[strum(to_string = "Rust")]
    RustTab,
    #[strum(to_string = "C++")]
    CXXTab,
    #[strum(to_string = "Zed")]
    ZedTab,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> eyre::Result<()> {
        while self.state == AppState::Running(RunningState::Main)
            || self.state == AppState::Running(RunningState::InTab)
        {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> eyre::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Right => self.next_tab(),
                    KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        if self.state == AppState::Running(RunningState::Main) {
            self.selected_tab = self.selected_tab.next();
        }
    }

    pub fn previous_tab(&mut self) {
        if self.state == AppState::Running(RunningState::Main) {
            self.selected_tab = self.selected_tab.previous();
        }
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl SelectedTab {
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);

        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf);

        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "ZZ: manage your dev environment".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::PythonTab => self.render_tab0(area, buf),
            Self::RustTab => self.render_tab1(area, buf),
            Self::CXXTab => self.render_tab2(area, buf),
            Self::ZedTab => self.render_tab3(area, buf),
        }
    }
}

impl SelectedTab {
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render_tab0(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Hello, World!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab1(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Manage your Rust installation here!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab2(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Manage your CXX installation here!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab3(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Install the most wonderful code editor here!")
            .block(self.block())
            .render(area, buf);
    }

    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::PythonTab => tailwind::GRAY,
            Self::RustTab => tailwind::RED,
            Self::CXXTab => tailwind::RED,
            Self::ZedTab => tailwind::RED,
        }
    }
}
