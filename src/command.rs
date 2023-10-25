pub struct Command {
    command: &'static str,
    output: &'static str,
}

impl Command {
    pub const fn new(command: &'static str, output: &'static str) -> Self {
        Self { command, output }
    }
}
