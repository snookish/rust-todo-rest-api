use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Todo {
    pub fn new(id: u64, title: impl Into<String>, description: impl Into<String>) -> Self {
        let now = chrono::Utc::now().naive_utc();

        Self {
            id,
            title: title.into(),
            description: description.into(),
            is_completed: false,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTodoResponse {
    pub todos: Vec<Todo>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: String,
}
