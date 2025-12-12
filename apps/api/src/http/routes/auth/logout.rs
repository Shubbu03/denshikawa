use axum::{extract::State, Json};
use serde::Deserialize;

use crate::auth::{jwt, AuthError, CurrentUser};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    #[serde(default)]
    pub refresh_token: Option<String>,
}

pub async fn logout(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<LogoutRequest>,
) -> Result<axum::http::StatusCode, AuthError> {
    // If refresh token provided, revoke it specifically
    if let Some(refresh_token) = req.refresh_token {
        let token_hash = jwt::hash_refresh_token(&refresh_token);

        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE token_hash = $1 AND user_id = $2 AND revoked_at IS NULL
            "#,
        )
        .bind(&token_hash)
        .bind(user.id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AuthError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
    } else {
        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE user_id = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(user.id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AuthError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
}
