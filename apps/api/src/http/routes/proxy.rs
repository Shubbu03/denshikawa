use axum::{
    body::Body,
    extract::{Query, State},
    http::{header, HeaderValue, StatusCode},
    response::Response,
};
use reqwest::Client;
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct ProxyQuery {
    url: String,
}

pub async fn proxy_image(
    Query(params): Query<ProxyQuery>,
    State(_state): State<AppState>,
) -> Result<Response<Body>, (StatusCode, String)> {
    if !params.url.starts_with("https://uploads.mangadex.org/")
        && !params.url.contains("mangadex.network")
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid URL: Only MangaDex image URLs are allowed".to_string(),
        ));
    }

    let client = Client::new();
    let response = client.get(&params.url).send().await.map_err(|e| {
        (
            StatusCode::BAD_GATEWAY,
            format!("Failed to fetch image: {}", e),
        )
    })?;

    if !response.status().is_success() {
        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Upstream returned status: {}", response.status()),
        ));
    }

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("image/jpeg")
        .to_string();

    let bytes = response.bytes().await.map_err(|e| {
        (
            StatusCode::BAD_GATEWAY,
            format!("Failed to read image: {}", e),
        )
    })?;

    let response_builder = Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_str(&content_type).unwrap(),
        )
        .header(header::CACHE_CONTROL, "public, max-age=31536000")
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*");

    Ok(response_builder
        .body(Body::from(bytes.to_vec()))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build response: {}", e),
            )
        })?)
}
