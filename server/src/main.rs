use tokio::net::TcpListener;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let app = server::router();
    let listener = TcpListener::bind("0.0.0.0:1234").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
