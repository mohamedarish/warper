use std::io::{stderr, Stderr};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

struct Terminal {
    stderr: Stderr,
}

impl Default for Terminal {
    fn default() -> Self {
        Self { stderr: stderr() }
    }
}

impl Terminal {
    pub fn startup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stderr.execute(EnterAlternateScreen)?;
        execute!(self.stderr, EnterAlternateScreen)?;
        execute!(self.stderr, EnableMouseCapture)?;
        enable_raw_mode()?;

        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stderr.execute(LeaveAlternateScreen)?;
        execute!(self.stderr, LeaveAlternateScreen)?;
        execute!(self.stderr, DisableMouseCapture)?;
        disable_raw_mode()?;

        Ok(())
    }
}
