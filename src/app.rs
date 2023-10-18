enum FocusedApp {
    InputBox,
    HistoryBar,
    OutputBox,
}

#[derive(Default)]
pub struct App {
    should_quit: bool,
    focused_app: FocusedApp,
}

impl Default for FocusedApp {
    fn default() -> Self {
        Self::InputBox
    }
}
