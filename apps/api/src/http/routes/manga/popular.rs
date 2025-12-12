use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::mangadex::MangaDexError;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct PopularQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    20
}

#[derive(serde::Serialize)]
pub struct PopularResponse {
    pub data: Vec<MangaSummary>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

#[derive(serde::Serialize)]
pub struct MangaSummary {
    pub id: String,
    pub mangadex_id: String,
    pub title: String,
    pub cover_url: String,
    pub status: String,
}

pub async fn get_popular_manga(
    Query(params): Query<PopularQuery>,
    State(state): State<AppState>,
) -> Result<Json<PopularResponse>, MangaDexError> {
    let limit = params.limit.min(100);

    let response = state
        .mangadex_client
        .get_popular_manga(limit, params.offset)
        .await?;

    let summaries: Vec<MangaSummary> = response
        .data
        .into_iter()
        .filter_map(|m| {
            let attrs = &m.attributes;
            let title = attrs
                .title
                .en
                .clone()
                .or_else(|| attrs.title.ja.clone())
                .or_else(|| {
                    attrs
                        .title
                        .other
                        .values()
                        .find_map(|v| v.as_str().map(|s| s.to_string()))
                })
                .unwrap_or_else(|| "Untitled".to_string());

            let cover_url = m
                .relationships
                .iter()
                .find(|r| r.rel_type == "cover_art")
                .and_then(|r| r.attributes.as_ref())
                .and_then(|a| a.file_name.as_ref())
                .map(|filename| {
                    format!("https://uploads.mangadex.org/covers/{}/{}", m.id, filename)
                })
                .unwrap_or_default();

            Some(MangaSummary {
                id: m.id.clone(),
                mangadex_id: m.id,
                title,
                cover_url,
                status: attrs.status.clone(),
            })
        })
        .collect();

    Ok(Json(PopularResponse {
        total: response.total.unwrap_or(summaries.len() as u32),
        limit,
        offset: params.offset,
        data: summaries,
    }))
}
