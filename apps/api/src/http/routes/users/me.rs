use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::FromRow;

use crate::auth::CurrentUser;
use crate::AppState;

#[derive(Serialize, FromRow)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_me(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<UserResponse>, axum::http::StatusCode> {
    let user_data = sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT
            id::text AS id,
            email,
            username,
            role::text AS role,
            created_at::text AS created_at,
            updated_at::text AS updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|err| {
        tracing::error!("failed to fetch user {}: {}", user.id, err);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match user_data {
        Some(user) => Ok(Json(user)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}
