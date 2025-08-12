use serde::{Deserialize, Serialize};

// src/models/task.rs
/// Represents a Todoist task with minimal fields for local and API use.
#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize, // Local ID for TUI
    pub todoist_id: Option<String>, // Tododist API ID (null for local-only tasks)
    pub title: String, // Task content (e.g., "Buy Milk")
    pub checked: bool, // Completion status
}

impl Task {
    /// Creates a new task with the given ID, title, and completion status.
    pub fn new(id: usize, title: &str, checked: bool) -> Self {
        Task {
            id,
            todoist_id: None,
            title: title.to_string(),
            checked,
        }
    }

    /// Creates a task from Todoist API data with a local ID.
    pub fn from_api(id: usize, todoist_id: String, title: String, checked: bool) -> Self {
        Task {
            id,
            todoist_id: Some(todoist_id),
            title,
            checked,
        }
    }
}