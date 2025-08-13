use serde::{Deserialize, Serialize};

// src/models/task.rs
/// Represents a Todoist task with minimal fields for local and API use.
#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize, // Local ID for TUI
    pub todoist_id: String, // Todoist API ID
    pub title: String, // Task content (e.g., "Buy Milk")
    pub is_completed: bool, // Completion status
}

impl Task {
    /// Creates a new task with the given ID, title, and completion status.
    pub fn new(id: usize, title: &str, is_completed: bool) -> Self {
        Task {
            id,
            todoist_id: "".to_string(),
            title: title.to_string(),
            is_completed,
        }
    }
}