use std::io::{stderr, Stderr};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Frame, Layout, Rect},
    widgets::Paragraph,
    Terminal,
};

use crate::command::CommandStore;

#[derive(Default)]
struct Size {
    width: u16,
    height: u16,
}

pub struct Backend {
    size: Size,
    stderr: Stderr,
    terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl Default for Backend {
    fn default() -> Self {
        Self {
            size: Size::default(),
            stderr: stderr(),
            terminal: Terminal::new(CrosstermBackend::new(stderr()))
                .expect("Cannot initialize the terminal"),
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

    pub fn ui(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| {
            let width = f.size().width;
            let height = f.size().height;

            self.size = Size { width, height };

            let layout = Layout::default()
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(f.size());
        })?;

        Ok(())
    }

    fn draw_left(
        self,
        frame: &mut Frame<CrosstermBackend<Stderr>>,
        commands: Vec<CommandStore>,
        selected_item: usize,
        size: Rect,
    ) {
        let layout = Layout::default()
            .constraints([
                Constraint::Percentage(90),
                Constraint::Percentage(8),
                Constraint::Percentage(2),
            ])
            .split(size);
    }

    fn draw_output(
        self,
        frame: &mut Frame<CrosstermBackend<Stderr>>,
        commands: &[CommandStore],
        selected_item: usize,
        size: Rect,
    ) {
        frame.render_widget(Paragraph::new(commands[selected_item].output()), size);
    }
}
