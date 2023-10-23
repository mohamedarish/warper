use std::io::stderr;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

struct Terminal {}

impl Terminal {
    pub fn startup() -> Result<(), Box<dyn std::error::Error>> {
        stderr().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        Ok(())
    }

    pub fn shutdown() -> Result<(), Box<dyn std::error::Error>> {
        stderr().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}
