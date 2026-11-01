use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

impl Todo {
    pub fn new(id: u64, title: impl Into<String>, description: impl Into<String>) -> Self {
        return Self {
            id,
            title: title.into(),
            description: description.into(),
            is_completed: false,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTodoResponse {
    pub todos: Vec<Todo>,
}
