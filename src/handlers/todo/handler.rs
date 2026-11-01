use crate::handlers::todo::models::{CreateTodoRequest, ListTodoResponse, Todo};
use axum::{Json, extract::Path, http::StatusCode};
use std::sync::{LazyLock, Mutex};

static TODOS: LazyLock<Mutex<Vec<Todo>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub struct Handler;

impl Handler {
    pub async fn list() -> (StatusCode, Json<ListTodoResponse>) {
        let todos = TODOS.lock().unwrap();

        (
            StatusCode::OK,
            Json(ListTodoResponse {
                todos: todos.clone(),
            }),
        )
    }

    pub async fn get_by_id(Path(id): Path<u64>) -> Result<Json<Todo>, StatusCode> {
        let todos = TODOS.lock().unwrap();
        let todo = todos.iter().find(|todo| todo.id == id).cloned();

        match todo {
            Some(todo) => Ok(Json(todo)),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    pub async fn create(
        Json(payload): Json<CreateTodoRequest>,
    ) -> (StatusCode, Json<ListTodoResponse>) {
        let mut todos = TODOS.lock().unwrap();
        let id = (todos.len() + 1) as u64;

        todos.push(Todo {
            id,
            title: payload.title,
            description: payload.description,
            is_completed: false,
        });

        (
            StatusCode::CREATED,
            Json(ListTodoResponse {
                todos: todos.clone(),
            }),
        )
    }
}
