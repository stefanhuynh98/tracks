use axum::Router;
use axum::routing::get;

use crate::handlers::auth::handle_github_callback;

pub fn auth_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello from /v1/auth!" }))
        .route("/provider/github", get(handle_github_callback))
}
