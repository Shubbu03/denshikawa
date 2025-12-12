use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;

use crate::mangadex::{cache::get_chapters_with_cache, MangaDexError};
use crate::AppState;

#[derive(Serialize)]
pub struct NavigationResponse {
    pub prev_chapter_id: Option<String>,
    pub next_chapter_id: Option<String>,
    pub current_chapter_id: String,
}

pub async fn get_chapter_navigation(
    Path(chapter_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<NavigationResponse>, MangaDexError> {
    let chapter = state.mangadex_client.get_chapter(&chapter_id).await?;

    let manga_id = chapter
        .relationships
        .iter()
        .find(|r| r.rel_type == "manga")
        .map(|r| r.id.clone())
        .ok_or_else(|| {
            MangaDexError::ApiError("Chapter does not have a manga relationship".to_string())
        })?;

    let chapters = get_chapters_with_cache(
        &manga_id,
        "en",
        &state.db_pool,
        &state.mangadex_client,
        &state.mangadex_config,
    )
    .await?;

    let current_index = chapters
        .iter()
        .position(|c| c.mangadex_id == chapter_id)
        .ok_or_else(|| {
            MangaDexError::ApiError("Chapter not found in manga chapter list".to_string())
        })?;

    let prev_chapter_id = if current_index > 0 {
        Some(chapters[current_index - 1].mangadex_id.clone())
    } else {
        None
    };

    let next_chapter_id = if current_index < chapters.len() - 1 {
        Some(chapters[current_index + 1].mangadex_id.clone())
    } else {
        None
    };

    Ok(Json(NavigationResponse {
        prev_chapter_id,
        next_chapter_id,
        current_chapter_id: chapter_id,
    }))
}
