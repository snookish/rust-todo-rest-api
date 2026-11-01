use axum::Router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5001").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
