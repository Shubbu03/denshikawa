use crate::config::AuthConfig;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub auth_config: AuthConfig,
}
