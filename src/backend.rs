use std::io::{stderr, Stderr};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

#[derive(Default)]
pub struct Size {
    width: u16,
    height: u16,
}

pub struct Backend {
    size: Size,
    pub stderr: Stderr,
}

impl Default for Backend {
    fn default() -> Self {
        Self {
            size: Size::default(),
            stderr: stderr(),
        }
    }
}

impl Backend {
    pub fn update_size(&mut self, width: u16, height: u16) {
        self.size = Size { width, height };
    }
    pub fn startup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(self.stderr, EnterAlternateScreen)?;
        execute!(self.stderr, EnableMouseCapture)?;
        enable_raw_mode()?;

        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(self.stderr, LeaveAlternateScreen)?;
        execute!(self.stderr, DisableMouseCapture)?;
        disable_raw_mode()?;

        Ok(())
    }
}
