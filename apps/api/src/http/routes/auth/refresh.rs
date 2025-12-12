use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{jwt, AuthError};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub tokens: TokenResponse,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, AuthError> {
    // Verify refresh token
    let claims = jwt::verify_refresh_token(&req.refresh_token, &state.auth_config.jwt_secret)?;

    if claims.token_type != "refresh" {
        return Err(AuthError::TokenInvalid);
    }

    // Check if token is revoked
    let token_hash = jwt::hash_refresh_token(&req.refresh_token);
    let token_record = sqlx::query_as::<_, (bool, bool)>(
        r#"
        SELECT 
            revoked_at IS NOT NULL AS is_revoked,
            expires_at < NOW() AS is_expired
        FROM refresh_tokens
        WHERE token_hash = $1
        "#,
    )
    .bind(&token_hash)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    let (is_revoked, is_expired) = match token_record {
        Some((revoked, expired)) => (revoked, expired),
        None => return Err(AuthError::TokenInvalid),
    };

    if is_revoked {
        return Err(AuthError::TokenRevoked);
    }

    if is_expired {
        return Err(AuthError::TokenExpired);
    }

    // Revoke old refresh token
    sqlx::query(
        r#"
        UPDATE refresh_tokens
        SET revoked_at = NOW()
        WHERE token_hash = $1
        "#,
    )
    .bind(&token_hash)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("Failed to revoke token: {}", e)))?;

    // Get user info
    let user = sqlx::query_as::<_, (Uuid, String, String)>(
        r#"
        SELECT id, email, role::text
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(claims.sub)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    let (user_id, email, role) = user;

    // Issue new token pair (token rotation)
    let new_refresh_token_id = Uuid::new_v4();
    let access_token = jwt::issue_access_token(
        user_id,
        &email,
        &role,
        &state.auth_config.jwt_secret,
        state.auth_config.access_token_ttl_secs,
    )?;

    let refresh_token = jwt::issue_refresh_token(
        user_id,
        new_refresh_token_id,
        &state.auth_config.jwt_secret,
        state.auth_config.refresh_token_ttl_days,
    )?;

    // Store new refresh token
    let new_token_hash = jwt::hash_refresh_token(&refresh_token);
    let expires_at = chrono::Utc::now()
        + chrono::Duration::days(state.auth_config.refresh_token_ttl_days);

    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(new_refresh_token_id)
    .bind(user_id)
    .bind(&new_token_hash)
    .bind(expires_at)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("Failed to store refresh token: {}", e)))?;

    Ok(Json(RefreshResponse {
        tokens: TokenResponse {
            access_token,
            refresh_token,
            expires_in: state.auth_config.access_token_ttl_secs,
        },
    }))
}
