pub mod chapters;
pub mod get_manga;
pub mod latest;
pub mod popular;
pub mod search;

use axum::{routing::get, Router};

use crate::AppState;

pub fn manga_routes() -> Router<AppState> {
    Router::new()
        .route("/search", get(search::search_manga))
        .route("/popular", get(popular::get_popular_manga))
        .route("/latest", get(latest::get_latest_manga))
        .route("/{id}", get(get_manga::get_manga))
        .route("/{id}/chapters", get(chapters::get_chapters))
}
