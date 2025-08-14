// src/cli/commands.rs
use clap::{Parser, Subcommand};
use crate::utils::error::AppResult;
use crate::controller::app::App;

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
        /// Task completion status
        checked: bool,
    },
    /// Deletes a task
    Delete {
        /// Task ID
        id: usize,
    },
}

/// Processes CLI commands and updates the app state.
pub async fn process_command(app: &mut App, command: &Commands) -> AppResult<()> {
    match command {
        Commands::Add { title } => app.add_task(title).await,
        Commands::Update { id, title, checked } => app.update_task(*id, title, *checked).await,
        Commands::Delete { id } => app.delete_task(*id).await,
    }
}