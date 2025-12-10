use tracing::{Level};
use tracing_subscriber::{FmtSubscriber};

use std::net::SocketAddr;
mod config;
mod http;

use crate::{config::AppConfig, http::build_router};

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

    // 4. Build HTTP router
    let app = build_router(&config);

    // 5. Bind TCP listener and serve
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
