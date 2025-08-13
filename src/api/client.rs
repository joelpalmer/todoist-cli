use crate::models::task::Task;
use crate::utils::error::AppResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct TasksResponse {
    results: Vec<TaskResponse>,
}

#[derive(Deserialize)]
struct TaskResponse {
    id: String,
    description: String, // Adjust to content if confirmed by curl
    checked: bool,
}

#[derive(Deserialize)]
struct CreatedTaskResponse {
    // Wrapper for POST /tasks response, adjust based on actual structure
    item: TaskResponse,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
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
            .await?;

        let status = response.status();
        let raw_text = response.text().await?;

        if !status.is_success() {
            match serde_json::from_str::<ErrorResponse>(&raw_text) {
                Ok(error_response) => {
                    return Err(anyhow::anyhow!("API error: {}", error_response.error));
                }
                Err(_) => {
                    return Err(anyhow::anyhow!("Non-success status {}: {}", status, raw_text));
                }
            }
        }

        let tasks_response: TasksResponse = serde_json::from_str(&raw_text).map_err(|e| {
            anyhow::anyhow!("Failed to deserialize tasks: {}. Raw response: {}", e, raw_text)
        })?;

        let tasks = tasks_response
            .results
            .into_iter()
            .enumerate()
            .map(|(i, item)| Task {
                id: i + 1,
                todoist_id: item.id,
                title: item.description,
                checked: item.checked,
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
            .json(&json!({ "description": title })) // Adjust to content if confirmed
            .send()
            .await?;

        let status = response.status();
        let raw_text = response.text().await?;

        if !status.is_success() {
            match serde_json::from_str::<ErrorResponse>(&raw_text) {
                Ok(error_response) => {
                    return Err(anyhow::anyhow!("API error: {}", error_response.error));
                }
                Err(_) => {
                    return Err(anyhow::anyhow!("Non-success status {}: {}", status, raw_text));
                }
            }
        }

        let created_response: CreatedTaskResponse = serde_json::from_str(&raw_text).map_err(|e| {
            anyhow::anyhow!("Failed to deserialize created task: {}. Raw response: {}", e, raw_text)
        })?;

        Ok(Task {
            id: 0, // Local ID set by caller
            todoist_id: created_response.item.id,
            title: created_response.item.description,
            checked: created_response.item.checked,
        })
    }

    /// Updates a task in Todoist.
    pub async fn update_task(&self, todoist_id: &str, title: &str) -> AppResult<()> {
        self.client
            .patch(format!("https://api.todoist.com/api/v1/tasks/{}", todoist_id))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&json!({ "description": title })) // Adjust to content if confirmed
            .send()
            .await?;
        Ok(())
    }

    /// Deletes a task in Todoist.
    pub async fn delete_task(&self, todoist_id: &str) -> AppResult<()> {
        self.client
            .delete(format!("https://api.todoist.com/api/v1/tasks/{}", todoist_id))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;
        Ok(())
    }
}