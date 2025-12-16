use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::mangadex::MangaDexError;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct AggregateQuery {
    #[serde(default)]
    pub lang: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AggregateResponse {
    pub volumes: serde_json::Value,
}

pub async fn get_manga_aggregate(
    Path(mangadex_id): Path<String>,
    Query(params): Query<AggregateQuery>,
    State(state): State<AppState>,
) -> Result<Json<AggregateResponse>, MangaDexError> {
    let lang_ref = params.lang.as_deref();
    let aggregate_data = state
        .mangadex_client
        .get_manga_aggregate(mangadex_id.as_str(), lang_ref)
        .await?;

    Ok(Json(AggregateResponse {
        volumes: aggregate_data,
    }))
}
