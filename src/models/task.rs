/// Represents a Todoist task (subset to start with)
/// Mirrors Todoist API's `content` and `checked` properties.
/// https://developer.todoist.com/api/v1/#tag/Tasks
#[derive(Clone)]
pub struct Task {
    pub title:String, // Task content (e.g., "Work on Todoist CLI")
    pub checked:bool, // Completion status of the task
}

impl Task {
    /// Creates a new Task with the given title and completion status.
    pub fn new(title:String, checked:bool) -> Task {
        Task {
            title: title.to_string(),
            checked
        }
    }
}