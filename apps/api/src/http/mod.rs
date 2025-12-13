use axum::{
    http::{header, Method},
    routing::{delete, get, post, put},
    Router,
};
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use crate::{config::AppConfig, AppState};

pub mod routes;

pub fn build_router(config: &AppConfig, state: AppState) -> Router {
    // CORS configuration for frontend (localhost:3000)
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    Router::new()
        .route("/health", get(routes::health::ping))
        .route("/users/{id}", get(routes::get_user_by_id::get_user_by_id))
        .route("/users/me", get(routes::users::me::get_me))
        .nest("/auth", routes::auth::auth_routes(&config.auth))
        .nest("/manga", routes::manga::manga_routes())
        .nest("/chapters", routes::chapters::chapter_routes())
        .route("/proxy/image", get(routes::proxy::proxy_image))
        .route(
            "/users/me/bookmarks",
            get(routes::users::bookmarks::get_bookmarks),
        )
        .route(
            "/users/me/bookmarks/{manga_id}",
            post(routes::users::bookmarks::add_bookmark),
        )
        .route(
            "/users/me/bookmarks/{manga_id}",
            delete(routes::users::bookmarks::remove_bookmark),
        )
        .route(
            "/users/me/library",
            get(routes::users::library::get_library),
        )
        .route(
            "/users/me/progress",
            get(routes::users::progress::get_all_progress),
        )
        .route(
            "/users/me/progress/{manga_id}",
            get(routes::users::progress::get_progress),
        )
        .route(
            "/users/me/progress/{manga_id}",
            put(routes::users::progress::update_progress),
        )
        .route(
            "/users/me/history",
            get(routes::users::history::get_history),
        )
        .route(
            "/users/me/history/{chapter_id}",
            post(routes::users::history::mark_chapter_read),
        )
        .route(
            "/users/me/history/{chapter_id}",
            delete(routes::users::history::remove_from_history),
        )
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
