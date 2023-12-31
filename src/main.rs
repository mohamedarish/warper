use std::{env, process::Command};

use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyModifiers,
    KeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Terminal},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};

#[derive(PartialEq)]
enum FocusArea {
    InputArea,
    OutputArea,
    History,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lotrem_ipsum = Vec::<String>::new();

    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    crossterm::execute!(
        std::io::stderr(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    )?;
    crossterm::execute!(std::io::stderr(), EnableMouseCapture)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut frame_size = (0, 0);

    let mut focused = FocusArea::InputArea;
    let mut item_selected = 0;
    let mut current_command_number = 0;

    let mut command = String::new();

    let mut list_filled_with_items = Vec::<String>::new();
    let mut new_old_command = String::new();

    let mut right = 0;
    let mut bottom = 0;

    let current_dir = env::current_dir().expect("Cannot access current directory");
    let current_director = current_dir.to_str().expect("Cannot convert to string");
    let mut current_directory = current_director.split('/').collect::<Vec<&str>>();
    loop {
        terminal.draw(|f| {
            right = f.size().width * 8 / 10;
            bottom = f.size().height * 9 / 10;

            frame_size = (f.size().width, f.size().height);
            let side_chart = Layout::default()
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .direction(Direction::Horizontal)
                .split(f.size());
            let down_chart = Layout::default()
                .constraints([
                    Constraint::Percentage(90),
                    Constraint::Percentage(7),
                    Constraint::Percentage(3),
                ])
                .split(side_chart[0]);
            if focused == FocusArea::OutputArea && !lotrem_ipsum.is_empty() {
                f.render_widget(
                    Paragraph::new(lotrem_ipsum[item_selected].clone()).block(
                        Block::default()
                            .title("Paragraph")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(Color::Yellow)),
                    ),
                    down_chart[0],
                );
            } else if !lotrem_ipsum.is_empty() {
                f.render_widget(
                    Paragraph::new(lotrem_ipsum[item_selected].clone()).block(
                        Block::default()
                            .title("Paragraph")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    ),
                    down_chart[0],
                );
            } else if focused == FocusArea::OutputArea {
                f.render_widget(
                    Block::default()
                        .title("Paragraph")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Yellow)),
                    down_chart[0],
                );
            } else {
                f.render_widget(
                    Block::default()
                        .title("Paragraph")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                    down_chart[0],
                );
            }

            let mut list_containing_items = Vec::<ListItem>::new();

            for item in list_filled_with_items.clone() {
                list_containing_items.push(ListItem::new(item));
            }

            let items = List::new(list_containing_items)
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::REVERSED)
                        .add_modifier(Modifier::SLOW_BLINK),
                )
                .highlight_symbol("> ");

            if focused == FocusArea::InputArea {
                f.render_widget(
                    Paragraph::new(command.clone()).block(
                        Block::default()
                            .title(current_directory.join("/"))
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(Color::Yellow)),
                    ),
                    down_chart[1],
                );
            } else {
                f.render_widget(
                    Paragraph::new(command.clone()).block(
                        Block::default()
                            .title(current_directory.join("/"))
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    ),
                    down_chart[1],
                );
            }

            if focused == FocusArea::History && !list_filled_with_items.is_empty() {
                f.render_stateful_widget(
                    items.block(
                        Block::default()
                            .title("History List")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(Color::Yellow)),
                    ),
                    side_chart[1],
                    &mut ListState::default().with_selected(Some(item_selected)),
                );
            } else if !list_filled_with_items.is_empty() {
                f.render_stateful_widget(
                    items.block(
                        Block::default()
                            .title("History")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    ),
                    side_chart[1],
                    &mut ListState::default().with_selected(Some(item_selected)),
                );
            } else if focused == FocusArea::History {
                f.render_widget(
                    Block::default()
                        .title("History List")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Yellow)),
                    side_chart[1],
                );
            } else {
                f.render_widget(
                    Block::default()
                        .title("History")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                    side_chart[1],
                );
            }
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(500))? {
            match crossterm::event::read()? {
                event::Event::Key(key) => {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        if focused == FocusArea::History {
                            let KeyEvent { code, .. } = key;

                            match code {
                                KeyCode::Up => {
                                    if item_selected < 1 {
                                        item_selected = lotrem_ipsum.len() - 1;
                                    } else {
                                        item_selected -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    item_selected += 1;
                                    item_selected %= lotrem_ipsum.len();
                                }
                                _ => {}
                            }
                        }

                        if focused == FocusArea::InputArea {
                            if let KeyEvent {
                                code,
                                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                                ..
                            } = key
                            {
                                match code {
                                    KeyCode::Enter => {
                                        if command == "exit" {
                                            break;
                                        }

                                        if command == "clear" {
                                            list_filled_with_items.clear();
                                            lotrem_ipsum.clear();
                                            command.clear();
                                            current_command_number = 0;
                                            continue;
                                        }

                                        if command.starts_with("cd") {
                                            let split_command =
                                                command.split(' ').collect::<Vec<&str>>();

                                            let dir = split_command[1];

                                            if dir == ".." {
                                                current_directory.pop();
                                            }
                                            command = String::new();
                                            continue;
                                        }

                                        if command == "pwd" {
                                            list_filled_with_items.push(command.clone());
                                            lotrem_ipsum.push(current_directory.join("/"));
                                            command = String::new();
                                            continue;
                                        }

                                        list_filled_with_items.push(command.clone());

                                        let mut split_command =
                                            command.split(' ').collect::<Vec<&str>>();

                                        let command_to_run = Command::new(split_command[0])
                                            .args(&mut split_command[1..])
                                            .current_dir(current_directory.join("/"))
                                            .output()
                                            .map_or(
                                                Command::new("echo")
                                                    .arg(format!(
                                                        "command execution failed: {command}"
                                                    ))
                                                    .output()
                                                    .expect("No way this throws an error"),
                                                |f| f,
                                            );

                                        let mut command_output =
                                            String::from_utf8(command_to_run.stdout)
                                                .expect("Cannot get output");

                                        if command_output.is_empty() {
                                            command_output =
                                                String::from_utf8(command_to_run.stderr)
                                                    .expect("Cannot get error");
                                        }

                                        lotrem_ipsum.push(command_output);

                                        item_selected = lotrem_ipsum.len() - 1;
                                        command = String::new();
                                        current_command_number = item_selected + 1;
                                    }
                                    KeyCode::Backspace => {
                                        command.pop();
                                    }
                                    KeyCode::Char(c) => {
                                        command.push(c);
                                    }
                                    KeyCode::Up => {
                                        if current_command_number == list_filled_with_items.len() {
                                            new_old_command = command.clone();
                                        }
                                        current_command_number =
                                            current_command_number.saturating_sub(1);
                                        if !list_filled_with_items.is_empty() {
                                            command = list_filled_with_items
                                                [current_command_number]
                                                .clone();
                                        }
                                    }
                                    KeyCode::Down => {
                                        if current_command_number < list_filled_with_items.len() {
                                            current_command_number += 1;
                                        }

                                        command = if current_command_number
                                            == list_filled_with_items.len()
                                        {
                                            new_old_command.clone()
                                        } else {
                                            list_filled_with_items[current_command_number].clone()
                                        };
                                    }
                                    KeyCode::Right => {
                                        current_command_number = list_filled_with_items.len();

                                        command = new_old_command.clone();
                                    }
                                    _ => {}
                                }
                            }
                        }

                        if let KeyEvent {
                            code: KeyCode::Char(c),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        } = key
                        {
                            match c {
                                'k' => focused = FocusArea::OutputArea,
                                'j' => focused = FocusArea::InputArea,
                                'l' => focused = FocusArea::History,
                                'q' => break,
                                _ => {}
                            }

                            continue;
                        }
                    }
                }
                event::Event::Mouse(action) => {
                    if action.kind == event::MouseEventKind::Down(event::MouseButton::Left) {
                        let row = action.row;
                        let column = action.column;

                        if column < right {
                            if row < bottom {
                                focused = FocusArea::OutputArea;
                            } else {
                                focused = FocusArea::InputArea;
                            }
                        } else if !list_filled_with_items.is_empty() {
                            focused = FocusArea::History;
                            item_selected = (row - 1) as usize;
                            if item_selected >= lotrem_ipsum.len() {
                                item_selected = lotrem_ipsum.len() - 1;
                            }
                        } else {
                            focused = FocusArea::History;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    crossterm::execute!(std::io::stderr(), DisableMouseCapture)?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
