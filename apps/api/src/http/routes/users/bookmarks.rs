use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use sqlx::FromRow;

use crate::auth::CurrentUser;
use crate::AppState;

#[derive(Serialize, FromRow)]
pub struct BookmarkResponse {
    pub id: String,
    pub manga_mangadex_id: String,
    pub created_at: String,
}

pub async fn get_bookmarks(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<Vec<BookmarkResponse>>, StatusCode> {
    let bookmarks = sqlx::query_as::<_, BookmarkResponse>(
        r#"
        SELECT 
            id::text AS id,
            manga_mangadex_id,
            created_at::text AS created_at
        FROM user_bookmarks
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user.id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(bookmarks))
}

pub async fn add_bookmark(
    Path(manga_mangadex_id): Path<String>,
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        r#"
        INSERT INTO user_bookmarks (user_id, manga_mangadex_id)
        VALUES ($1, $2)
        ON CONFLICT (user_id, manga_mangadex_id) DO NOTHING
        "#,
    )
    .bind(user.id)
    .bind(&manga_mangadex_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn remove_bookmark(
    Path(manga_mangadex_id): Path<String>,
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        r#"
        DELETE FROM user_bookmarks
        WHERE user_id = $1 AND manga_mangadex_id = $2
        "#,
    )
    .bind(user.id)
    .bind(&manga_mangadex_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
