// src/api/client.rs
use crate::models::task::Task;
use crate::utils::error::AppResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct TaskResponse {
    id: String,
    content: String,
    is_completed: bool,
}

/// Todoist REST v1 API client.
pub struct ApiClient {
    client: Client,
    token: String,
}

impl ApiClient {
    /// Creates a new API client with the given Todoist token.
    pub fn new(token: String) -> Self {
        ApiClient {
            client: Client::new(),
            token,
        }
    }

    /// Fetches tasks from the Todoist REST v1 API.
    pub async fn fetch_tasks(&self) -> AppResult<Vec<Task>> {
        let response = self
            .client
            .get("https://api.todoist.com/api/v1/tasks")
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .json::<Vec<TaskResponse>>()
            .await?;

        let tasks = response
            .into_iter()
            .enumerate()
            .map(|(i, item)| Task {
                id: i + 1, // Assign local ID
                todoist_id: item.id,
                title: item.content,
                is_completed: item.is_completed,
            })
            .collect();
        Ok(tasks)
    }

    /// Adds a task to Todoist and returns the new task.
    pub async fn add_task(&self, title: &str) -> AppResult<Task> {
        let response = self
            .client
            .post("https://api.todoist.com/api/v1/tasks")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&json!({"content": title}))
            .send()
            .await?
            .json::<TaskResponse>()
            .await?;

        Ok(Task {
            id: 0, // Local ID set by caller
            todoist_id: response.id,
            title: response.content,
            is_completed: response.is_completed,
        })
    }

    /// Updates a task in Todoist.
    pub async fn update_task(&self, todoist_id: &str, title: &str) -> AppResult<()> {
        self
            .client
            .patch(&format!("https://api.todoist.com/api/v1/tasks/{}", todoist_id))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&json!({"content": title}))
            .send()
            .await?;
        Ok(())
    }

    /// Deletes a task in Todoist.
    pub async fn delete_task(&self, todoist_id: &str) -> AppResult<()> {
        self
            .client
            .delete(&format!("https://api.todoist.com/api/v1/tasks/{}", todoist_id))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;
        Ok(())
    }
}