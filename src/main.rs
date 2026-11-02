use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::db::pool::create_db_pool;
use handlers::todo;

mod db;
mod handlers;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = create_db_pool()?;
    let _todo_service = service::todo::Service::new(pool);

    let router = Router::new()
        .route("/todos", get(todo::handler::Handler::list))
        .route("/todos", post(todo::handler::Handler::create))
        .route("/todos/{id}", get(todo::handler::Handler::get_by_id))
        .route("/todos/{id}", delete(todo::handler::Handler::delete_by_id));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5001").await?;

    println!("Listening on: http://localhost:5001");
    axum::serve(listener, router).await?;

    Ok(())
}
