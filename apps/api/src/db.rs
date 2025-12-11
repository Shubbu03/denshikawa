use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

use crate::config::AppConfig;

pub async fn create_pool(config: &AppConfig) -> Result<PgPool> {
    let max_connections: u32 = std::env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let connect_timeout_secs: u64 = std::env::var("DB_CONNECT_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(15);

    let idle_timeout_secs: u64 = std::env::var("DB_IDLE_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(600);

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(connect_timeout_secs))
        .idle_timeout(Duration::from_secs(idle_timeout_secs))
        .connect(&config.database_url)
        .await
        .context("Failed to connect to Postgres")?;

    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool)
        .await
        .context("Postgres probe failed")?;

    Ok(pool)
}