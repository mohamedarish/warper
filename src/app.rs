use std::io::Stderr;

use ratatui::{
    prelude::{Constraint, CrosstermBackend, Layout, Rect},
    widgets::Paragraph,
    Frame, Terminal,
};

use crate::{backend::Backend, command::CommandStore};

#[derive(Default)]
pub enum FocusedApp {
    #[default]
    InputBox,
    HistoryBar,
    OutputBox,
}

pub struct App {
    should_quit: bool,
    focused_app: FocusedApp,
    current_selection: usize,
    backend: Backend,
    commands: Vec<CommandStore>,
    terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl Default for App {
    fn default() -> Self {
        let backend = Backend::default();

        let terminal = Terminal::new(CrosstermBackend::new(backend.stderr))
            .expect("Cannot initialize the terminal");

        Self {
            should_quit: false,
            focused_app: FocusedApp::default(),
            current_selection: 0,
            backend: Backend::default(),
            commands: Vec::<CommandStore>::new(),
            terminal,
        }
    }
}

impl App {
    pub fn insert_new_command(&mut self, command: CommandStore) {
        self.commands.push(command);
    }

    pub fn change_focus(&mut self, new_focus: FocusedApp) {
        self.focused_app = new_focus;
    }

    pub fn change_selection(&mut self, new_index: usize) {
        self.current_selection = new_index;
    }

    pub fn increment_selection(&mut self) {
        self.current_selection += 1;
        self.current_selection %= self.commands.len() - 1; // TODO this one is to be removed if the
                                                           // TODO current command will not be stored in the Vector.
    }

    pub fn decrement_selection(&mut self) {
        if self.current_selection > 0 {
            self.current_selection -= 1;
        } else {
            self.current_selection = self.commands.len() - 2; // TODO change this to 1 if the
                                                              // TODO current command is not stored in the vector
        }
    }

    pub fn toggle_quit(&mut self) {
        self.should_quit = true;
    }

    pub fn update_terminal_size(&mut self, width: u16, height: u16) {
        self.backend.update_size(width, height);
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    pub fn ui(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut width = 0;
        let mut height = 0;
        self.terminal.draw(|f| {
            width = f.size().width;
            height = f.size().height;

            let layout = Layout::default()
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(f.size());
        })?;

        self.update_terminal_size(width, height);

        Ok(())
    }

    fn draw_left(
        &self,
        frame: &mut Frame<CrosstermBackend<Stderr>>,
        current_command: CommandStore,
        commands: &[CommandStore],
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

        self.draw_output(frame, commands, selected_item, layout[0]);
        self.draw_input(frame, current_command, layout[1])
    }

    fn draw_output(
        &self,
        frame: &mut Frame<CrosstermBackend<Stderr>>,
        commands: &[CommandStore],
        selected_item: usize,
        size: Rect,
    ) {
        frame.render_widget(Paragraph::new(commands[selected_item].output()), size);
        // TODO put it
        // in a block
    }

    fn draw_input(
        &self,
        frame: &mut Frame<CrosstermBackend<Stderr>>,
        current_command: CommandStore,
        size: Rect,
    ) {
        frame.render_widget(Paragraph::new(current_command.command()), size); // TODO put it in a
                                                                              // block
    }
}
