#[derive(Default)]
pub struct CommandStore {
    command: &'static str,
    output: &'static str,
}

impl CommandStore {
    pub const fn new(command: &'static str, output: &'static str) -> Self {
        Self { command, output }
    }

    pub const fn command(&self) -> &'static str {
        self.command
    }

    pub const fn output(&self) -> &'static str {
        self.output
    }
}
