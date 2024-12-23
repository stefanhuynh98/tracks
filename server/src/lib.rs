mod error;
mod models;
mod routes;
mod handlers;
mod auth;
mod oauth;

use std::sync::Arc;

use axum::{Extension, Router};
use sqlx::MySqlPool;
use routes::v1;

pub struct Context {
    pub pool: MySqlPool,
}

pub fn init_router(ctx: Arc<Context>) -> Router {
    Router::new()
        .nest("/v1", v1::router())
        .layer(Extension(ctx))
}
