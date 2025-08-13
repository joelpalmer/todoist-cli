use serde::{Deserialize, Serialize};

/// Represents a Todoist task with minimal fields for local CRUD.
#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub todoist_id: String,
    pub title: String,
    pub checked: bool,
}

impl Task {
    /// Creates a new task with the given local ID, title, and completion status.
    pub fn new(id: usize, title: &str, checked: bool) -> Self {
        Task {
            id,
            todoist_id: "".to_string(),
            title: title.to_string(),
            checked,
        }
    }
}