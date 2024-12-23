use anyhow::{Result, Context};
use axum_test::TestServer;
use sqlx::{MySql, Pool};
use dotenvy::dotenv;

use server::{init_router, Context};

pub async fn setup() -> Result<(TestServer, Pool<MySql>)> {
    dotenv().ok();
    let db_url = std::env::var("TEST_DATABASE_URL").context("TEST_DATABASE_URL")?;
    let pool   = Pool::connect(&db_url).await?;
    let state  = Context { pool };
    let app    = init_router(state);
    let server = TestServer::new(app)?;

    Ok((server, app.pool))
}
