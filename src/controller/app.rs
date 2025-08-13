use crate::api::client::ApiClient;
use crate::db::cache::Cache;
use crate::models::task::Task;
use crate::utils::error::AppResult;
use ratatui::widgets::ListState;

/// Application mode: Normal (navigation), InsertAdd (adding new task), or InsertEdit (editing task).
#[derive(PartialEq)]
pub enum Mode {
    Normal,
    InsertAdd,
    InsertEdit,
}

/// Application state managing tasks and TUI mode.
pub struct App {
    tasks: Vec<Task>,
    next_id: usize,
    list_state: ListState,
    mode: Mode,
    pub input_buffer: String,
    api_client: ApiClient,
    cache: Cache,
}

impl App {
    /// Initializes the app with API client and cache.
    pub fn new(token: String) -> AppResult<Self> {
        let cache = Cache::new()?;
        let mut tasks = cache.load_tasks()?;
        if tasks.is_empty() {
            tasks = vec![
                Task::new(1, "Buy Milk", false),
                Task::new(2, "Write Code", false),
                Task::new(3, "Fix Bugs", false),
            ];
        }
        let mut list_state = ListState::default();
        if !tasks.is_empty() {
            list_state.select(Some(0));
        }
        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Ok(App {
            tasks,
            next_id,
            list_state,
            mode: Mode::Normal,
            input_buffer: String::new(),
            api_client: ApiClient::new(token),
            cache,
        })
    }

    /// Syncs tasks with the Todoist API and updates cache.
    pub async fn sync_tasks(&mut self) -> AppResult<()> {
        let api_tasks = self.api_client.fetch_tasks().await?;
        let mut tasks = Vec::new();
        for (i, mut task) in api_tasks.into_iter().enumerate() {
            task.id = self.next_id + i;
            tasks.push(task);
        }
        self.next_id += tasks.len();
        self.tasks = tasks;
        self.cache.save_tasks(&self.tasks)?;
        if !self.tasks.is_empty() && self.list_state.selected().is_none() {
            self.list_state.select(Some(0));
        }
        Ok(())
    }

    /// Adds a new task locally and to Todoist.
    pub async fn add_task(&mut self, title: &str) -> AppResult<()> {
        if !title.trim().is_empty() {
            let mut task = self.api_client.add_task(title).await?;
            task.id = self.next_id;
            self.tasks.push(task);
            self.next_id += 1;
            self.list_state.select(Some(self.tasks.len() - 1));
            self.cache.save_tasks(&self.tasks)?;
        }
        Ok(())
    }

    /// Updates a task locally and in Todoist.
    pub async fn update_task(&mut self, id: usize, title: &str) -> AppResult<()> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if !title.trim().is_empty() {
                self.api_client.update_task(&task.todoist_id, title).await?;
                task.title = title.to_string();
                self.cache.save_tasks(&self.tasks)?;
            }
        }
        Ok(())
    }

    /// Deletes a task locally and in Todoist.
    pub async fn delete_task(&mut self, id: usize) -> AppResult<()> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            let task = &self.tasks[index];
            self.api_client.delete_task(&task.todoist_id).await?;
            self.tasks.remove(index);
            self.cache.save_tasks(&self.tasks)?;
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
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
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
                self.input_buffer = task.title.clone();
            }
        }
        self.mode = Mode::InsertEdit;
    }

    /// Exits Insert mode, saving the input as a new or updated task.
    pub async fn exit_insert_mode(&mut self) -> AppResult<()> {
        let input = self.input_buffer.clone();
        if !input.trim().is_empty() {
            match self.mode {
                Mode::InsertAdd => {
                    self.add_task(&input).await?;
                }
                Mode::InsertEdit => {
                    if let Some(i) = self.list_state.selected() {
                        if let Some(task) = self.tasks.get(i) {
                            self.update_task(task.id, &input).await?;
                        } else {
                            self.add_task(&input).await?;
                        }
                    } else {
                        self.add_task(&input).await?;
                    }
                }
                Mode::Normal => {}
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
}