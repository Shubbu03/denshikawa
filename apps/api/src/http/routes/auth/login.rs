use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::{verify_password, AuthError};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, FromRow)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub tokens: TokenResponse,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthError> {
    // Find user by email
    let user = sqlx::query_as::<_, (Uuid, String, String, String, String)>(
        r#"
        SELECT id, email, username, password, role::text
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(&req.email)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("Database error: {}", e)))?
    .ok_or(AuthError::InvalidCredentials)?;

    let (user_id, email, username, password_hash, role) = user;

    // Verify password
    if !verify_password(&req.password, &password_hash)? {
        return Err(AuthError::InvalidCredentials);
    }

    // Issue tokens
    let refresh_token_id = Uuid::new_v4();
    let access_token = crate::auth::jwt::issue_access_token(
        user_id,
        &email,
        &role,
        &state.auth_config.jwt_secret,
        state.auth_config.access_token_ttl_secs,
    )?;

    let refresh_token = crate::auth::jwt::issue_refresh_token(
        user_id,
        refresh_token_id,
        &state.auth_config.jwt_secret,
        state.auth_config.refresh_token_ttl_days,
    )?;

    // Store refresh token hash in DB
    let token_hash = crate::auth::jwt::hash_refresh_token(&refresh_token);
    let expires_at = chrono::Utc::now()
        + chrono::Duration::days(state.auth_config.refresh_token_ttl_days);

    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(refresh_token_id)
    .bind(user_id)
    .bind(&token_hash)
    .bind(expires_at)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("Failed to store refresh token: {}", e)))?;

    Ok(Json(LoginResponse {
        user: UserResponse {
            id: user_id.to_string(),
            email,
            username,
            role,
            created_at: String::new(),
        },
        tokens: TokenResponse {
            access_token,
            refresh_token,
            expires_in: state.auth_config.access_token_ttl_secs,
        },
    }))
}
