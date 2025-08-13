// src/db/cache.rs
use crate::models::task::Task;
use crate::utils::error::AppResult;
use rusqlite::{Connection, params};

/// SQLite cache for tasks.
pub struct Cache {
    conn: Connection,
}

impl Cache {
    /// Initializes the SQLite database and creates the tasks table.
    pub fn new() -> AppResult<Self> {
        let conn = Connection::open("tasks.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                todoist_id TEXT NOT NULL,
                title TEXT NOT NULL,
                is_completed INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(Cache { conn })
    }

    /// Saves tasks to the database, replacing existing ones.
    pub fn save_tasks(&self, tasks: &[Task]) -> AppResult<()> {
        self.conn.execute("DELETE FROM tasks", [])?;
        for task in tasks {
            self.conn.execute(
                "INSERT INTO tasks (id, todoist_id, title, is_completed) VALUES (?1, ?2, ?3, ?4)",
                params![task.id, task.todoist_id, task.title, task.is_completed as i32],
            )?;
        }
        Ok(())
    }

    /// Loads tasks from the database.
    pub fn load_tasks(&self) -> AppResult<Vec<Task>> {
        let mut stmt = self.conn.prepare("SELECT id, todoist_id, title, is_completed FROM tasks")?;
        let tasks = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    todoist_id: row.get(1)?,
                    title: row.get(2)?,
                    is_completed: row.get::<_, i32>(3)? != 0,
                })
            })?
            .collect::<Result<Vec<Task>, rusqlite::Error>>()?;
        Ok(tasks)
    }
}