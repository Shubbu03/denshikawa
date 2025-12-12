use crate::auth::error::AuthError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub email: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: Uuid,
    pub jti: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub fn issue_access_token(
    user_id: Uuid,
    email: &str,
    role: &str,
    secret: &str,
    ttl_secs: i64,
) -> Result<String, AuthError> {
    let now = Utc::now();
    let claims = AccessTokenClaims {
        sub: user_id,
        email: email.to_string(),
        role: role.to_string(),
        exp: (now + Duration::seconds(ttl_secs)).timestamp(),
        iat: now.timestamp(),
        token_type: "access".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("JWT encoding failed: {}", e)))
}

pub fn issue_refresh_token(
    user_id: Uuid,
    token_id: Uuid,
    secret: &str,
    ttl_days: i64,
) -> Result<String, AuthError> {
    let now = Utc::now();
    let claims = RefreshTokenClaims {
        sub: user_id,
        jti: token_id,
        exp: (now + Duration::days(ttl_days)).timestamp(),
        iat: now.timestamp(),
        token_type: "refresh".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AuthError::Internal(anyhow::anyhow!("JWT encoding failed: {}", e)))
}

pub fn verify_access_token(token: &str, secret: &str) -> Result<AccessTokenClaims, AuthError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;

    decode::<AccessTokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
        _ => AuthError::TokenInvalid,
    })
}

pub fn verify_refresh_token(token: &str, secret: &str) -> Result<RefreshTokenClaims, AuthError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;

    decode::<RefreshTokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
        _ => AuthError::TokenInvalid,
    })
}

pub fn hash_refresh_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}
