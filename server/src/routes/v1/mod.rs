mod auth;
mod teams;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/teams", teams::router())
}
