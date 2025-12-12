use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::mangadex::{cache::get_chapters_with_cache, MangaDexError};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ChaptersQuery {
    #[serde(default = "default_lang")]
    pub lang: String,
}

fn default_lang() -> String {
    "en".to_string()
}

pub async fn get_chapters(
    Path(mangadex_id): Path<String>,
    Query(params): Query<ChaptersQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::manga::Chapter>>, MangaDexError> {
    let chapters = get_chapters_with_cache(
        &mangadex_id,
        &params.lang,
        &state.db_pool,
        &state.mangadex_client,
        &state.mangadex_config,
    )
    .await?;

    Ok(Json(chapters))
}

