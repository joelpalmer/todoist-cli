// src/api/client.rs
use crate::models::task::Task;
use crate::utils::error::AppResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

/// Todoist Sync API client.
pub struct ApiClient {
    client: Client,
    token: String,
    sync_token: String, // Tracks sync state
}

#[derive(Serialize, Deserialize)]
struct SyncResponse {
    items: Option<Vec<TaskResponse>>,
    sync_token: String,
}

#[derive(Serialize, Deserialize)]
struct TaskResponse {
    id: String,
    content: String,
    checked: bool,
}

impl ApiClient {
    /// Creates a new API client with the given Todoist token.
    pub fn new(token: String) -> Self {
        ApiClient {
            client: Client::new(),
            token,
            sync_token: "*".to_string(), // Initial sync token
        }
    }

    /// Fetches tasks from the Todoist Sync API.
    pub async fn fetch_tasks(&mut self) -> AppResult<Vec<Task>> {
        let response = self
            .client
            .post("https://api.todoist.com/sync/v9/sync")
            .form(&[
                ("token", &self.token),
                ("sync_token", &self.sync_token),
                ("resource_types", &json!(["items"]).to_string()),
            ])
            .send()
            .await?
            .json::<SyncResponse>()
            .await?;

        self.sync_token = response.sync_token;

        let tasks = response
            .items
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(i, item)| Task::from_api(i + 1, item.id, item.content, item.checked))
            .collect();
        Ok(tasks)
    }

    /// Adds a task to Todoist and returns the new task.
    pub async fn add_task(&self, title: &str) -> AppResult<Task> {
        let command_id = Uuid::new_v4().to_string();
        let command = json!({
            "type": "item_add",
            "temp_id": command_id,
            "uuid": Uuid::new_v4().to_string(),
            "args": {
                "content": title,
                "checked": false
            }
        });

        let response = self
            .client
            .post("https://api.todoist.com/sync/v9/sync")
            .form(&[
                ("token", &self.token),
                ("sync_token", &self.sync_token),
                ("commands", &json!([command]).to_string()),
            ])
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let todoist_id = response["temp_id_mapping"][&command_id]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to get task ID"))?
            .to_string();

        Ok(Task::from_api(0, todoist_id, title.to_string(), false)) // ID will be set by caller
    }
}