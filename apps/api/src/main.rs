use tracing::{Level};
use tracing_subscriber::{FmtSubscriber};

use std::net::SocketAddr;
mod auth;
mod config;
mod http;
mod db;

use crate::{config::AppConfig, http::build_router};
mod state;
pub use state::AppState;

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

    // 4b. Run migrations (configurable)
    let run_migrations = std::env::var("RUN_MIGRATIONS_ON_STARTUP")
        .map(|v| v != "0" && v.to_lowercase() != "false")
        .unwrap_or(true);
    if run_migrations {
        tracing::info!("Running database migrations...");
        sqlx::migrate!().run(&pool).await?;
        tracing::info!("Migrations applied");
    }

    // 5. App state
    let state = AppState {
        db_pool: pool,
        auth_config: config.auth.clone(),
    };

    // 6. Build HTTP router
    let app = build_router(&config, state.clone());

    // 7. Bind TCP listener and serve
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await?;

    Ok(())
}
