use std::sync::Arc;

use crate::config::AuthConfig;
use crate::mangadex::MangaDexClient;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub auth_config: AuthConfig,
    pub mangadex_client: Arc<MangaDexClient>,
    pub mangadex_config: crate::config::MangaDexConfig,
}
