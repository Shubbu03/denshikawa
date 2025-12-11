use axum::{Json, extract::State, response::IntoResponse};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub async fn ping(State(_state): State<AppState>) -> impl IntoResponse {
    Json(HealthResponse { status: "pong" })
}
