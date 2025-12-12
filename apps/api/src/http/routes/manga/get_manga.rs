use axum::{
    extract::{Path, State},
    Json,
};

use crate::mangadex::{cache::get_manga_with_cache, MangaDexError};
use crate::AppState;

pub async fn get_manga(
    Path(mangadex_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<crate::manga::Manga>, MangaDexError> {
    let manga = get_manga_with_cache(
        &mangadex_id,
        &state.db_pool,
        &state.mangadex_client,
        &state.mangadex_config,
    )
    .await?;

    Ok(Json(manga))
}

