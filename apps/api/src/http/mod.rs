use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::{config::AppConfig, AppState};

pub mod routes;

pub fn build_router(config: &AppConfig, state: AppState) -> Router {
    Router::new()
        .route("/health", get(routes::health::ping))
        .route("/users/{id}", get(routes::get_user_by_id::get_user_by_id))
        .route("/users/me", get(routes::users::me::get_me))
        .nest("/auth", routes::auth::auth_routes(&config.auth))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
