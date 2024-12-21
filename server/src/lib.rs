pub mod app;
pub mod error;
pub mod routes;
pub mod handlers;
pub mod auth;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/v1", routes::v1_router())
}
