use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::AppState;

#[derive(Serialize, FromRow)]
pub struct GetUserByIdResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_user_by_id(
    Path(user_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<GetUserByIdResponse>, StatusCode> {
    let user = sqlx::query_as::<_, GetUserByIdResponse>(
        r#"
        SELECT
            id::text AS id,
            email,
            username,
            created_at::text AS created_at,
            updated_at::text AS updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|err| {
        tracing::error!("failed to fetch user {}: {}", user_id, err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::NOT_FOUND),
    }
}