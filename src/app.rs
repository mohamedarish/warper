#[derive(Default)]
enum FocusedApp {
    #[default]
    InputBox,
    HistoryBar,
    OutputBox,
}

#[derive(Default)]
pub struct App {
    should_quit: bool,
    focused_app: FocusedApp,
    current_selection: usize,
    // terminal: Terminal,
    // commands: &'static Command
}

// impl App {}
