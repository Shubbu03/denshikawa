use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::{AppState, config::AppConfig};

pub mod routes;

pub fn build_router(_config: &AppConfig, state: AppState) -> Router {
    Router::new()
        .route("/ping", get(routes::health::ping))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
