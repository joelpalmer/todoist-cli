// src/cli/commands.rs
use clap::{Parser, Subcommand};
use crate::utils::error::AppResult;

/// CLI arguments for the Todoist CLI.
#[derive(Parser)]
#[command(name = "todoist-cli")]
#[command(about = "A terminal-based Todoist client", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available CLI commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Adds a new task
    Add {
        /// Task title
        title: String,
    },
    /// Updates an existing task
    Update {
        /// Task ID
        id: usize,
        /// New task title
        title: String,
    },
    /// Deletes a task
    Delete {
        /// Task ID
        id: usize,
    },
}

/// Processes CLI commands and updates the app state.
pub fn process_command(app: &mut crate::controller::app::App, command: &Commands) -> AppResult<()> {
    match command {
        Commands::Add { title } => app.add_task(title),
        Commands::Update { id, title } => app.update_task(*id, title),
        Commands::Delete { id } => app.delete_task(*id),
    }
}