use std::sync::Arc;

use tokio::net::TcpListener;
use axum::serve;

use tracks::AppState;
use tracks::server::init_router;
use tracks::db::create_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        pool: create_pool()?,
    };

    let app = init_router(Arc::new(state));
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    serve(listener, app).await?;

    Ok(())
}
