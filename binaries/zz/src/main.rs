mod zz;

fn main() -> eyre::Result<()> {
    let terminal = ratatui::init();

    let app_result = zz::App {
        state: zz::AppState::Running,
        selected_tab: zz::SelectedTab::new_python(),
    }
    .run(terminal);

    ratatui::restore();

    app_result
}
