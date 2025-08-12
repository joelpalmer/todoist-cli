// src/controller/app.rs
use crate::models::task::Task;
use crate::utils::error::AppResult;
use ratatui::widgets::ListState;

/// Application mode: Normal (navigation), InsertAdd (adding new task), or InsertEdit (editing task).
#[derive(PartialEq)]
pub enum Mode {
    Normal,
    InsertAdd, // For adding new tasks
    InsertEdit, // For editing existing tasks
}

/// Application state managing tasks and TUI mode.
pub struct App {
    tasks: Vec<Task>, // In-memory task list
    next_id: usize, // Tracks next available task ID
    list_state: ListState, // Manages TUI list selection
    mode: Mode, // Current TUI mode
    input_buffer: String, // Buffer for task input in Insert mode
}

impl App {
    /// Initializes the app with hardcoded tasks.
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        App {
            tasks: vec![
                Task::new(1, "Add storage", false),
                Task::new(2, "Write Code", false),
                Task::new(3, "Fix Bugs", true),
            ],
            next_id: 4, // Next available ID
            list_state,
            mode: Mode::Normal,
            input_buffer: String::new(),
        }
    }

    /// Adds a new task with the given title.
    pub fn add_task(&mut self, title: &str) -> AppResult<()> {
        if !title.trim().is_empty() {
            self.tasks.push(Task::new(self.next_id, title.to_string().as_str(), false));
            self.next_id += 1;
            // Select the new task
            self.list_state.select(Some(self.tasks.len() - 1));
        }
        Ok(())
    }

    /// Updates the title of the task with the given ID.
    pub fn update_task(&mut self, id: usize, title: &str) -> AppResult<()> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if !title.trim().is_empty() {
                task.title = title.to_string();
            }
        }
        Ok(())
    }

    /// Deletes the task with the given ID.
    pub fn delete_task(&mut self, id: usize) -> AppResult<()> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            // Adjust selection if needed
            if self.tasks.is_empty() {
                self.list_state.select(None);
            } else if index <= self.list_state.selected().unwrap_or(0) {
                let new_index = self.list_state.selected().unwrap_or(1).saturating_sub(1);
                self.list_state.select(Some(new_index));
            }
        }
        Ok(())
    }

    /// Moves selection to the next task.
    pub fn next(&mut self) {
        if self.tasks.is_empty() {
            self.list_state.select(None);
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.tasks.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Moves selection to the previous task.
    pub fn previous(&mut self) {
        if self.tasks.is_empty() {
            self.list_state.select(None);
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => if i == 0 { self.tasks.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Enters Insert mode for adding a new task.
    pub fn enter_insert_add_mode(&mut self) {
        self.input_buffer.clear();
        self.mode = Mode::InsertAdd;
    }

    /// Enters Insert mode for editing the selected task.
    pub fn enter_insert_edit_mode(&mut self) {
        self.input_buffer.clear();
        if let Some(i) = self.list_state.selected() {
            if let Some(task) = self.tasks.get(i) {
                self.input_buffer = task.title.clone(); // Pre-fill with current task title
            }
        }
        self.mode = Mode::InsertEdit;
    }

    /// Exits Insert mode, saving the input as a new or updated task.
    pub fn exit_insert_mode(&mut self) -> AppResult<()> {
        // Copy input_buffer to avoid borrowing self immutably
        let input = self.input_buffer.clone();
        if !input.trim().is_empty() {
            match self.mode {
                Mode::InsertAdd => {
                    // Always add a new task
                    self.add_task(&input)?;
                }
                Mode::InsertEdit => {
                    if let Some(i) = self.list_state.selected() {
                        if let Some(task) = self.tasks.get(i) {
                            // Update existing task
                            self.update_task(task.id, &input)?;
                        } else {
                            // Fallback to adding if selection is invalid
                            self.add_task(&input)?;
                        }
                    } else {
                        // Add new task if no selection
                        self.add_task(&input)?;
                    }
                }
                Mode::Normal => {} // Should not happen
            }
        }
        self.mode = Mode::Normal;
        self.input_buffer.clear();
        Ok(())
    }

    /// Handles input in Insert mode.
    pub fn handle_input(&mut self, c: char) {
        self.input_buffer.push(c);
    }

    /// Handles backspace in Insert mode.
    pub fn handle_backspace(&mut self) {
        self.input_buffer.pop();
    }

    /// Gets the current mode.
    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    /// Gets the task list.
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Gets the mutable list state.
    pub fn list_state(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    /// Gets the input buffer.
    pub fn input_buffer(&self) -> &String {
        &self.input_buffer
    }
}