use axum::{Router, routing::get};
use handlers::todo;

mod handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = todo::models::Todo::new(1, "Learn Rust", "Be good at Rust");

    let router = Router::new()
        .route("/todos", get(todo::handler::Handler::list))
        .route("/todos/{id}", get(todo::handler::Handler::get_by_id));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5001").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
