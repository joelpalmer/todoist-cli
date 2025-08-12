// src/main.rs
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal, Frame,
};
use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use crate::utils::error::AppResult;
use clap::Parser;

mod models;
mod utils;
mod controller;
mod cli;

use controller::app::{App, Mode};
use cli::commands::{Cli, Commands, process_command};

/// Renders the TUI based on the app state.
fn render(f: &mut Frame, app: &mut App) {
    // Split the screen into two areas: task list (top) and input (bottom)
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Min(0), // Task list takes remaining space
            ratatui::layout::Constraint::Length(3), // Input field is 3 lines high
        ])
        .split(f.size());

    // Render task list
    let mode_str = match app.mode() {
        Mode::Normal => "Normal",
        Mode::InsertAdd => "Insert (Add)",
        Mode::InsertEdit => "Insert (Edit)",
    };
    let selected = app.list_state().selected();
    let items = app.tasks()
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let prefix = if Some(i) == selected { "> " } else { "  " };
            let status = if task.checked { "[x]" } else { "[ ]" };
            ListItem::new(format!("{} {} {}", prefix, status, task.title))
        })
        .collect::<Vec<_>>();
    let list = List::new(items)
        .block(Block::default()
            .title(format!("Todoist CLI [Mode: {}]", mode_str))
            .borders(Borders::ALL));
    f.render_stateful_widget(list, chunks[0], app.list_state());

    // Show input buffer in Insert modes
    if matches!(app.mode(), Mode::InsertAdd | Mode::InsertEdit) {
        let input_block = Block::default()
            .title("Input")
            .borders(Borders::ALL);
        let input = Paragraph::new(app.input_buffer().as_str())
            .block(input_block);
        f.set_cursor(
            chunks[1].x + 2 + app.input_buffer().len() as u16, // Cursor at end of input text
            chunks[1].y + 1, // Center vertically in input area
        );
        f.render_widget(input, chunks[1]);
    }
}

/// Runs the TUI application.
fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> AppResult<()> {
    loop {
        terminal.draw(|f| render(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match app.mode() {
                    Mode::Normal => match code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') => app.next(),
                        KeyCode::Char('k') => app.previous(),
                        KeyCode::Char('i') => app.enter_insert_edit_mode(),
                        KeyCode::Char('a') => app.enter_insert_add_mode(),
                        KeyCode::Char('d') => {
                            // Delete the selected task
                            if let Some(i) = app.list_state().selected() {
                                if let Some(task) = app.tasks().get(i) {
                                    app.delete_task(task.id)?;
                                }
                            }
                        }
                        _ => {}
                    },
                    Mode::InsertAdd | Mode::InsertEdit => match code {
                        KeyCode::Enter => app.exit_insert_mode()?,
                        KeyCode::Esc => app.exit_insert_mode()?,
                        KeyCode::Char(c) => app.handle_input(c),
                        KeyCode::Backspace => app.handle_backspace(),
                        _ => {}
                    },
                }
            }
        }
    }
    Ok(())
}

fn main() -> AppResult<()> {
    // Parse CLI arguments
    let cli = Cli::parse();
    let mut app = App::new();

    // Process CLI command if provided
    if let Some(command) = cli.command {
        process_command(&mut app, &command)?;
        return Ok(());
    }

    // Initialize terminal for TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run TUI
    let result = run_app(&mut terminal, &mut app);

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    result
}