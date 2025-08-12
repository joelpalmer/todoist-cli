// src/models/task.rs
/// Represents a Todoist task with minimal fields for local CRUD.
#[derive(Clone)]
pub struct Task {
    pub id: usize, // Unique ID for local tasks
    pub title: String, // Task content (e.g., "Buy Milk")
    pub checked: bool, // Completion status
}

impl Task {
    /// Creates a new task with the given ID, title, and completion status.
    pub fn new(id: usize, title: &str, checked: bool) -> Self {
        Task {
            id,
            title: title.to_string(),
            checked,
        }
    }
}