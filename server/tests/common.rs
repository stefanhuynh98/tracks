use std::env;
use axum_test::TestServer;
use dotenvy::dotenv;

pub fn setup() -> anyhow::Result<TestServer> {
    dotenv().ok();

    let app = server::router();
    let test_server = TestServer::new(app)?;

    Ok(test_server)
}
