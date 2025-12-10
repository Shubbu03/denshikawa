use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub async fn ping() -> impl IntoResponse {
    Json(HealthResponse { status: "pong" })
}
