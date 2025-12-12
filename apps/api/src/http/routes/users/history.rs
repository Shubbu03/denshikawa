use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::auth::CurrentUser;
use crate::AppState;

#[derive(Serialize, FromRow)]
pub struct ReadingHistoryItem {
    pub id: String,
    pub manga_mangadex_id: String,
    pub chapter_mangadex_id: String,
    pub read_at: String,
}

#[derive(Deserialize)]
pub struct HistoryQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}

pub async fn get_history(
    Query(params): Query<HistoryQuery>,
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<Vec<ReadingHistoryItem>>, StatusCode> {
    let history = sqlx::query_as::<_, ReadingHistoryItem>(
        r#"
        SELECT 
            id::text AS id,
            manga_mangadex_id,
            chapter_mangadex_id,
            read_at::text AS read_at
        FROM reading_history
        WHERE user_id = $1
        ORDER BY read_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user.id)
    .bind(params.limit)
    .bind(params.offset)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(history))
}

pub async fn mark_chapter_read(
    Path(chapter_id): Path<String>,
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<StatusCode, StatusCode> {
    let chapter = state
        .mangadex_client
        .get_chapter(&chapter_id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let manga_id = chapter
        .relationships
        .iter()
        .find(|r| r.rel_type == "manga")
        .map(|r| r.id.clone())
        .ok_or(StatusCode::BAD_REQUEST)?;

    sqlx::query(
        r#"
        INSERT INTO reading_history (user_id, manga_mangadex_id, chapter_mangadex_id)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, chapter_mangadex_id) 
        DO UPDATE SET read_at = NOW()
        "#,
    )
    .bind(user.id)
    .bind(&manga_id)
    .bind(&chapter_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn remove_from_history(
    Path(chapter_id): Path<String>,
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        r#"
        DELETE FROM reading_history
        WHERE user_id = $1 AND chapter_mangadex_id = $2
        "#,
    )
    .bind(user.id)
    .bind(&chapter_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
