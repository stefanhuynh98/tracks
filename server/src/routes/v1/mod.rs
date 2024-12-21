pub mod auth;

use axum::Router;
use axum::routing::get;

pub fn v1_router() -> Router {
    Router::new()
        .nest("/auth", auth::auth_router())
}
