use std::sync::{Arc, Once};

use anyhow::Result;
use dotenvy_macro::dotenv;
use sqlx::{Pool, MySql};
use axum_test::TestServer;

use tracks::AppState;
use tracks::server::init_router;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        tracing_subscriber::fmt::init();
    });
}

pub async fn setup() -> Result<(TestServer, Pool<MySql>)> {
    initialize();
    let db_url = dotenv!("TEST_DATABASE_URL");
    let pool   = Pool::connect(&db_url).await?;
    let state  = Arc::new(AppState { pool: pool.clone() });
    let app    = init_router(state);
    let server = TestServer::new(app)?;

    Ok((server, pool))
}
