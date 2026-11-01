use crate::handlers::todo::models::{ListTodoResponse, Todo};
use axum::{Json, extract::Path, http::StatusCode};

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

    pub async fn get_by_id(Path(id): Path<u64>) -> Result<Json<Todo>, StatusCode> {
        let todo = TODOS.iter().find(|todo| todo.id == id).cloned();
        match todo {
            Some(todo) => Ok(Json(todo)),
            None => Err(StatusCode::NOT_FOUND),
        }
    }
}
