use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::mangadex::MangaDexError;
use crate::manga::Manga;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    20
}

#[derive(serde::Serialize)]
pub struct SearchResponse {
    pub data: Vec<Manga>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

pub async fn search_manga(
    Query(params): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<SearchResponse>, MangaDexError> {
    let limit = params.limit.min(100);

    let response = state
        .mangadex_client
        .search_manga(&params.q, limit, params.offset)
        .await?;

    let manga: Vec<Manga> = response
        .data
        .into_iter()
        .filter_map(|m| Manga::try_from(m).ok())
        .collect();

    Ok(Json(SearchResponse {
        total: response.total.unwrap_or(manga.len() as u32),
        limit,
        offset: params.offset,
        data: manga,
    }))
}
