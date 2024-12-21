use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

use crate::routes::v1_router;

pub struct App {
    router: Router,
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new()
                .nest("/v1", v1_router()),
        }
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }

    pub async fn listen(&self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, self.router.clone()).await?;

        Ok(())
    }
}
