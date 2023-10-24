#![warn(clippy::nursery, clippy::unwrap_or_default, clippy::unwrap_used)]

use std::process::Command;

use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyModifiers,
    KeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Terminal},
    style::{Color, Modifier, Style, Stylize},
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

    let mut command = String::new();

    let mut list_filled_with_items = Vec::<ListItem>::new();
    loop {
        terminal.draw(|f| {
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
            } else {
                f.render_widget(
                    Block::default()
                        .title("Paragraph")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                    down_chart[0],
                )
            }
            let items = List::new(list_filled_with_items.clone())
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
                            .title("Command Input")
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
                            .title("Command Input")
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
                            .title("History List")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    ),
                    side_chart[1],
                    &mut ListState::default().with_selected(Some(item_selected)),
                );
            } else {
                f.render_widget(
                    Block::default()
                        .title("History List")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                    side_chart[1],
                )
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
                                        list_filled_with_items.push(ListItem::new(command.clone()));
                                        let mut split_command =
                                            command.split(' ').collect::<Vec<&str>>();
                                        lotrem_ipsum.push(
                                            String::from_utf8(
                                                Command::new(split_command[0])
                                                    .args(&mut split_command[1..])
                                                    .output()
                                                    .expect("Cannot run command")
                                                    .stdout,
                                            )
                                            .expect("Cannot get output"),
                                        );

                                        item_selected = lotrem_ipsum.len() - 1;
                                        command = String::new();
                                    }
                                    KeyCode::Backspace => {
                                        command.pop();
                                    }
                                    KeyCode::Char(c) => {
                                        command.push(c);
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

                        if column < 40 {
                            continue;
                        }

                        item_selected = (row - 1) as usize;
                        if item_selected >= lotrem_ipsum.len() {
                            item_selected = lotrem_ipsum.len() - 1;
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
