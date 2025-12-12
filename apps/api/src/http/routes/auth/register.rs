use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::auth::{hash_password, AuthError};
use crate::AppState;

lazy_static::lazy_static! {
    static ref USERNAME_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(
        length(min = 3, max = 30, message = "Username must be 3-30 characters"),
        custom(function = "validate_username_format")
    )]
    pub username: String,

    #[validate(
        length(min = 8, message = "Password must be at least 8 characters"),
        custom(function = "validate_password_strength")
    )]
    pub password: String,
}

fn validate_username_format(username: &str) -> Result<(), validator::ValidationError> {
    if !USERNAME_REGEX.is_match(username) {
        return Err(validator::ValidationError::new(
            "Username can only contain letters, numbers, and underscores",
        ));
    }
    Ok(())
}

fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(validator::ValidationError::new(
            "Password must contain uppercase, lowercase, and a number",
        ));
    }
    Ok(())
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
pub struct RegisterResponse {
    pub user: UserResponse,
    pub tokens: TokenResponse,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AuthError> {
    // Validate input
    req.validate()
        .map_err(|e| AuthError::ValidationError(e.to_string()))?;

    // Check if email already exists
    let email_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
    )
    .bind(&req.email)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    if email_exists {
        return Err(AuthError::EmailAlreadyExists);
    }

    // Check if username already exists
    let username_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
    )
    .bind(&req.username)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    if username_exists {
        return Err(AuthError::UsernameAlreadyExists);
    }

    // Hash password
    let password_hash = hash_password(&req.password)?;

    // Create user
    let user_id = Uuid::new_v4();
    let user = sqlx::query_as::<_, UserResponse>(
        r#"
        INSERT INTO users (id, email, username, password, role)
        VALUES ($1, $2, $3, $4, 'user')
        RETURNING id::text AS id, email, username, role::text AS role, created_at::text AS created_at
        "#,
    )
    .bind(user_id)
    .bind(&req.email)
    .bind(&req.username)
    .bind(&password_hash)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("Failed to create user: {}", e)))?;

    // Issue tokens
    let refresh_token_id = Uuid::new_v4();
    let access_token = crate::auth::jwt::issue_access_token(
        user_id,
        &req.email,
        "user",
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

    Ok(Json(RegisterResponse {
        user,
        tokens: TokenResponse {
            access_token,
            refresh_token,
            expires_in: state.auth_config.access_token_ttl_secs,
        },
    }))
}
