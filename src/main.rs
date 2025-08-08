use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use crate::utils::error::AppResult;

mod models;
mod utils;
use models::task::Task;

/// Application state holding tasks and the selected index.
struct App {
    tasks: Vec<Task>, // List of tasks to display
    list_state: ListState, // Manages selection in the task list
}

impl App {
    /// Initializes the app with hardcoded tasks for testing.
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0)); // Select first task by default
        App {
            tasks: vec![
                Task::new("Fix this constructor".parse().unwrap(), false),
                Task::new("Create a README for the haters".parse().unwrap(), false),
                Task::new("Add CRUD implementation".parse().unwrap(), true),
            ],
            list_state,
        }
    }

    /// Moves selection to the next task, wrapping around to the top.
    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.tasks.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Moves selection to the previous task, wrapping around to the bottom.
    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => if i == 0 { self.tasks.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

/// Runs the TUI application.
fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> AppResult<()> {
    let mut app = App::new();

    loop {
        // Render the TUI
        terminal.draw(|f| {
            // Create list items with selection indicator
            let items = app.tasks
                .iter()
                .enumerate()
                .map(|(i, task)| {
                    let prefix = if Some(i) == app.list_state.selected() { "> " } else { "  " };
                    let status = if task.checked { "[x]" } else { "[ ]" };
                    ListItem::new(format!("{} {} {}", prefix, status, task.title))
                })
                .collect::<Vec<_>>();
            let list = List::new(items)
                .block(Block::default().title("Joel's Todoist CLI").borders(Borders::ALL));
            f.render_stateful_widget(list, f.size(), &mut app.list_state);
        })?;

        // Handle input events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break, // Quit on 'q'
                    KeyCode::Char('j') => app.next(), // Move down
                    KeyCode::Char('k') => app.previous(), // Move up
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn main() -> AppResult<()> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app and capture any errors
    let result = run_app(&mut terminal);

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    result
}