use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::{config::AppConfig, AppState};

pub mod routes;

pub fn build_router(_config: &AppConfig, state: AppState) -> Router {
    Router::new()
        .route("/health", get(routes::health::ping))
        .route("/users/:id", get(routes::get_user_by_id::get_user_by_id))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
