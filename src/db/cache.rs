use crate::models::task::Task;
use crate::utils::error::AppResult;
use rusqlite::{Connection, params};

/// SQLite cache for tasks.
pub struct Cache {
    conn: Connection,
}

impl Cache {
    /// Initializes the SQLite database, creates the tasks table, and migrates schema if needed.
    pub fn new() -> AppResult<Self> {
        let conn = Connection::open("tasks.db")?;

        // Scope the PRAGMA query to release the borrow
        let columns = {
            let mut stmt = conn.prepare("PRAGMA table_info(tasks)")?;
            stmt.query_map([], |row| row.get::<_, String>(1))?
                .collect::<Result<Vec<String>, _>>()?
        };

        // Now conn is no longer borrowed
        if columns.contains(&"is_completed".to_string()) && !columns.contains(&"checked".to_string()) {
            // Migrate: Rename is_completed to checked
            conn.execute("ALTER TABLE tasks RENAME COLUMN is_completed TO checked", [])?;
        } else if !columns.contains(&"id".to_string()) {
            // Create table if it doesn't exist
            conn.execute(
                "CREATE TABLE IF NOT EXISTS tasks (
                    id INTEGER PRIMARY KEY,
                    todoist_id TEXT NOT NULL,
                    title TEXT NOT NULL,
                    checked INTEGER NOT NULL
                )",
                [],
            )?;
        }

        Ok(Cache { conn })
    }

    /// Saves tasks to the database, replacing existing ones.
    pub fn save_tasks(&self, tasks: &[Task]) -> AppResult<()> {
        self.conn.execute("DELETE FROM tasks", [])?;
        for task in tasks {
            self.conn.execute(
                "INSERT INTO tasks (id, todoist_id, title, checked) VALUES (?1, ?2, ?3, ?4)",
                params![task.id, task.todoist_id, task.title, task.checked as i32],
            )?;
        }
        Ok(())
    }

    /// Loads tasks from the database.
    pub fn load_tasks(&self) -> AppResult<Vec<Task>> {
        let mut stmt = self.conn.prepare("SELECT id, todoist_id, title, checked FROM tasks")?;
        let tasks = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    todoist_id: row.get(1)?,
                    title: row.get(2)?,
                    checked: row.get::<_, i32>(3)? != 0,
                })
            })?
            .collect::<Result<Vec<Task>, rusqlite::Error>>()?;
        Ok(tasks)
    }
}