use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MangaDexError {
    #[error("MangaDex API error: {0}")]
    ApiError(String),

    #[error("Rate limit exceeded")]
    RateLimited,

    #[error("Manga not found")]
    NotFound,

    #[error("Invalid response format")]
    InvalidResponse,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for MangaDexError {
    fn into_response(self) -> Response {
        let (status, error_code) = match &self {
            MangaDexError::ApiError(_) => (StatusCode::BAD_GATEWAY, "MANGADEX_API_ERROR"),
            MangaDexError::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED"),
            MangaDexError::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            MangaDexError::InvalidResponse => (StatusCode::BAD_GATEWAY, "INVALID_RESPONSE"),
            MangaDexError::NetworkError(_) => (StatusCode::BAD_GATEWAY, "NETWORK_ERROR"),
            MangaDexError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
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
