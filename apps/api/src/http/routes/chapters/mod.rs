pub mod navigation;
pub mod pages;

use axum::{routing::get, Router};

use crate::AppState;

pub fn chapter_routes() -> Router<AppState> {
    Router::new()
        .route("/{id}/pages", get(pages::get_chapter_pages))
        .route("/{id}/navigation", get(navigation::get_chapter_navigation))
}
