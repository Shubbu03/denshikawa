use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::auth::CurrentUser;
use crate::AppState;

#[derive(Serialize, FromRow)]
pub struct ReadingProgress {
    pub manga_mangadex_id: String,
    pub chapter_mangadex_id: String,
    pub page_number: i32,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct UpdateProgressRequest {
    pub chapter_id: String,
    pub page_number: u32,
}

pub async fn get_all_progress(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<Vec<ReadingProgress>>, StatusCode> {
    let progress = sqlx::query_as::<_, ReadingProgress>(
        r#"
        SELECT 
            manga_mangadex_id,
            chapter_mangadex_id,
            page_number,
            updated_at::text AS updated_at
        FROM user_reading_progress
        WHERE user_id = $1
        ORDER BY updated_at DESC
        "#,
    )
    .bind(user.id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(progress))
}

pub async fn get_progress(
    Path(manga_mangadex_id): Path<String>,
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<ReadingProgress>, StatusCode> {
    let progress = sqlx::query_as::<_, ReadingProgress>(
        r#"
        SELECT 
            manga_mangadex_id,
            chapter_mangadex_id,
            page_number,
            updated_at::text AS updated_at
        FROM user_reading_progress
        WHERE user_id = $1 AND manga_mangadex_id = $2
        "#,
    )
    .bind(user.id)
    .bind(&manga_mangadex_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match progress {
        Some(p) => Ok(Json(p)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_progress(
    Path(manga_mangadex_id): Path<String>,
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<UpdateProgressRequest>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        r#"
        INSERT INTO user_reading_progress (user_id, manga_mangadex_id, chapter_mangadex_id, page_number)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, manga_mangadex_id) 
        DO UPDATE SET 
            chapter_mangadex_id = EXCLUDED.chapter_mangadex_id,
            page_number = EXCLUDED.page_number,
            updated_at = NOW()
        "#,
    )
    .bind(user.id)
    .bind(&manga_mangadex_id)
    .bind(&req.chapter_id)
    .bind(req.page_number as i32)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
