use crate::{backend::Backend, command::CommandStore};

#[derive(Default)]
pub enum FocusedApp {
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
    backend: Backend,
    commands: Vec<CommandStore>,
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
}
