use tracing::{Level};
use tracing_subscriber::{FmtSubscriber};

use std::net::SocketAddr;
mod config;
mod http;
mod db;

use crate::{config::AppConfig, http::build_router};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load environment variables
    dotenvy::dotenv().ok();

    // 2. Initialize tracing (logging)
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set tracing subscriber");

    // 3. Build application configuration
    let config = AppConfig::from_env()?;
    let addr: SocketAddr = config.addr().parse()?;

    tracing::info!("Starting API server on {}", addr);

    // 4. Connecting to DB
    tracing::info!("Connecting to database...");
    let pool = db::create_pool(&config).await?;
    tracing::info!("Database connection established");

    // 5. App state
    let state = AppState { db_pool: pool };

    // 6. Build HTTP router
    let app = build_router(&config, state.clone());

    // 7. Bind TCP listener and serve
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
