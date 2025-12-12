use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Token expired")]
    TokenExpired,

    #[error("Token invalid")]
    TokenInvalid,

    #[error("Token revoked")]
    TokenRevoked,

    #[error("Missing authorization header")]
    MissingAuthHeader,

    #[error("Invalid authorization header format")]
    InvalidAuthHeader,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_code) = match &self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS"),
            AuthError::EmailAlreadyExists => (StatusCode::CONFLICT, "EMAIL_EXISTS"),
            AuthError::UsernameAlreadyExists => (StatusCode::CONFLICT, "USERNAME_EXISTS"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "TOKEN_EXPIRED"),
            AuthError::TokenInvalid => (StatusCode::UNAUTHORIZED, "TOKEN_INVALID"),
            AuthError::TokenRevoked => (StatusCode::UNAUTHORIZED, "TOKEN_REVOKED"),
            AuthError::MissingAuthHeader => (StatusCode::UNAUTHORIZED, "MISSING_AUTH"),
            AuthError::InvalidAuthHeader => (StatusCode::UNAUTHORIZED, "INVALID_AUTH"),
            AuthError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED"),
            AuthError::ValidationError(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            AuthError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        let body = serde_json::json!({
            "error": {
                "code": error_code,
                "message": self.to_string()
            }
        });

        (status, axum::Json(body)).into_response()
    }
}
