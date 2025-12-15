use axum::{extract::State, Json};
use serde::Serialize;

use crate::mangadex::MangaDexError;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct RandomMangaResponse {
    pub id: String,
}

pub async fn get_random_manga(
    State(state): State<AppState>,
) -> Result<Json<RandomMangaResponse>, MangaDexError> {
    let manga = state.mangadex_client.get_random_manga().await?;

    Ok(Json(RandomMangaResponse { id: manga.id }))
}


