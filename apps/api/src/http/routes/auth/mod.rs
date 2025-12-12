pub mod login;
pub mod logout;
pub mod refresh;
pub mod register;

use axum::{routing::post, Router};
use std::sync::Arc;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

use crate::{config::AuthConfig, AppState};

pub fn auth_routes(auth_config: &AuthConfig) -> Router<AppState> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1) // Replenish 1 request per second
            .burst_size(auth_config.rate_limit_requests) // Allow bursts up to rate_limit_requests
            .finish()
            .unwrap(),
    );

    Router::new()
        .route("/register", post(register::register))
        .route("/login", post(login::login))
        .route("/refresh", post(refresh::refresh))
        .route("/logout", post(logout::logout))
        .layer(GovernorLayer::new(governor_conf))
}
