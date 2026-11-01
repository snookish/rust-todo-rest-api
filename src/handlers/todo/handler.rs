use crate::handlers::todo::models::{ListTodoResponse, Todo};
use axum::{Json, http::StatusCode};

static TODOS: Vec<Todo> = vec![];

pub struct Handler;

impl Handler {
    pub async fn list() -> (StatusCode, Json<ListTodoResponse>) {
        return (
            StatusCode::OK,
            Json(ListTodoResponse {
                todos: TODOS.clone(),
            }),
        );
    }
}
