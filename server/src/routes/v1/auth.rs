use axum::Router;
use axum::routing::get;

use crate::handlers::auth::handle_github_callback;

pub fn router() -> Router {
    Router::new()
        .route("/", get(handle_github_callback))
}
