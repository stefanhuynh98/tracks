use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::handlers;
use crate::AppState;

pub fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        // OAuth callbacks (requests done by OAuth providers)
        .route("/v1/auth/provider/{provider}", get(handlers::auth::handle_oauth_callback))

        // Attach middleware
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http()))

        .with_state(state)
}
