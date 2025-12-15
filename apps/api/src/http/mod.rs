use axum::{
    http::{header, Method},
    routing::{delete, get, post, put},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{config::AppConfig, AppState};

pub mod routes;

pub fn build_router(config: &AppConfig, state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:3000"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    Router::new()
        .route("/health", get(routes::health::ping))
        .route("/users/{id}", get(routes::get_user_by_id::get_user_by_id))
        .route("/users/me", get(routes::users::me::get_me))
        .nest("/auth", routes::auth::auth_routes(&config.auth))
        .nest("/manga", routes::manga::manga_routes())
        .route("/proxy/image", get(routes::proxy::proxy_image))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
