use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::config::AppConfig;

pub mod routes;

pub fn build_router(_config: &AppConfig) -> Router {
    Router::new()
        .route("/ping", get(routes::health::ping))
        .layer(TraceLayer::new_for_http())
}
