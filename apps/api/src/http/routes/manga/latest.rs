use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::mangadex::MangaDexError;
use crate::manga::Manga;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct LatestQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    20
}

#[derive(serde::Serialize)]
pub struct LatestResponse {
    pub data: Vec<Manga>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

pub async fn get_latest_manga(
    Query(params): Query<LatestQuery>,
    State(state): State<AppState>,
) -> Result<Json<LatestResponse>, MangaDexError> {
    let limit = params.limit.min(100);

    let response = state
        .mangadex_client
        .get_latest_manga(limit, params.offset)
        .await?;

    let manga: Vec<Manga> = response
        .data
        .into_iter()
        .filter_map(|m| Manga::try_from(m).ok())
        .collect();

    Ok(Json(LatestResponse {
        total: response.total.unwrap_or(manga.len() as u32),
        limit,
        offset: params.offset,
        data: manga,
    }))
}
