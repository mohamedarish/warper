#![warn(
    clippy::nursery,
    // clippy::pedantic,
    clippy::unwrap_or_default,
    clippy::unwrap_used
)]

use std::process::Command;

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
    let lsal = Command::new("ls")
        .args(["-a", "-l"])
        .output()
        .expect("Cannot get output");
    let lsl = Command::new("ls")
        .arg("-l")
        .output()
        .expect("Cannot get output");
    let catctoml = Command::new("cat")
        .arg("Cargo.toml")
        .output()
        .expect("Cannot get output");
    let echo_something = Command::new("echo")
        .arg("Wow so many content: From echo")
        .output()
        .expect("Cannot get output");

    let lotrem_ipsum = [
        String::from_utf8(lsal.stdout).map_or(String::from("Cannot execute sw command -al"), |f| f),
        String::from_utf8(lsl.stdout).expect("No output"),
        String::from_utf8(catctoml.stdout).expect("No output"),
        String::from_utf8(echo_something.stdout).expect("No output"),
    ];
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    crossterm::execute!(
        std::io::stderr(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    )?;
    crossterm::execute!(std::io::stderr(), EnableMouseCapture)?;

    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Define our counter variable
    // This is the state of our application
    let mut counter = 0;

    let mut frame_size = (0, 0);

    let mut focused = FocusArea::InputArea;
    let mut item_selected = 0;
    // Main application loop
    loop {
        // Render the UI
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
            if focused == FocusArea::OutputArea {
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
            } else {
                f.render_widget(
                    Paragraph::new(lotrem_ipsum[item_selected].clone()).block(
                        Block::default()
                            .title("Paragraph")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    ),
                    down_chart[0],
                );
            }
            let items = List::new(vec![
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
                ListItem::new("Item 4"),
            ])
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::REVERSED)
                    .add_modifier(Modifier::SLOW_BLINK),
            )
            .highlight_symbol("> ");

            if focused == FocusArea::InputArea {
                f.render_widget(
                    Paragraph::new(format!("Counter 2: {counter}")).block(
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
                    Paragraph::new(format!("Counter 2: {counter}")).block(
                        Block::default()
                            .title("Command Input")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    ),
                    down_chart[1],
                );
            }

            if focused == FocusArea::History {
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
            } else {
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
            }
        })?;

        // Check for user input every 250 milliseconds
        if crossterm::event::poll(std::time::Duration::from_millis(500))? {
            match crossterm::event::read()? {
                event::Event::Key(key) => {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        if focused == FocusArea::History {
                            let KeyEvent { code, .. } = key;

                            match code {
                                KeyCode::Up => {
                                    if item_selected < 1 {
                                        item_selected = 3;
                                    } else {
                                        item_selected -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    item_selected += 1;
                                    item_selected %= 4;
                                }
                                _ => {}
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
                                _ => {}
                            }

                            continue;
                        }

                        match key.code {
                            crossterm::event::KeyCode::Char('j') => counter += 1,
                            crossterm::event::KeyCode::Char('k') => counter -= 1,
                            crossterm::event::KeyCode::Char('q') => break,
                            _ => {}
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
                        if item_selected > 3 {
                            item_selected = 3;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // shutdown down: reset terminal back to original state

    crossterm::execute!(std::io::stderr(), DisableMouseCapture)?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
