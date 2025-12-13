use anyhow::{Context, Result};
use std::env;

pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub auth: AuthConfig,
    pub mangadex: MangaDexConfig,
}

#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub access_token_ttl_secs: i64,
    pub refresh_token_ttl_days: i64,
    pub password_min_length: usize,
    pub rate_limit_requests: u32,
    pub rate_limit_window_secs: u64,
}

impl AuthConfig {
    pub fn from_env() -> Result<Self> {
        let jwt_secret = env::var("JWT_SECRET")
            .context("JWT_SECRET must be set (use a strong random string)")?;

        let access_token_ttl_secs = env::var("ACCESS_TOKEN_TTL_SECS")
            .unwrap_or_else(|_| "900".to_string())
            .parse::<i64>()
            .context("ACCESS_TOKEN_TTL_SECS must be a valid i64")?;

        let refresh_token_ttl_days = env::var("REFRESH_TOKEN_TTL_DAYS")
            .unwrap_or_else(|_| "7".to_string())
            .parse::<i64>()
            .context("REFRESH_TOKEN_TTL_DAYS must be a valid i64")?;

        let password_min_length = env::var("PASSWORD_MIN_LENGTH")
            .unwrap_or_else(|_| "8".to_string())
            .parse::<usize>()
            .context("PASSWORD_MIN_LENGTH must be a valid usize")?;

        let rate_limit_requests = env::var("RATE_LIMIT_REQUESTS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()
            .context("RATE_LIMIT_REQUESTS must be a valid u32")?;

        let rate_limit_window_secs = env::var("RATE_LIMIT_WINDOW_SECS")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<u64>()
            .context("RATE_LIMIT_WINDOW_SECS must be a valid u64")?;

        Ok(Self {
            jwt_secret,
            access_token_ttl_secs,
            refresh_token_ttl_days,
            password_min_length,
            rate_limit_requests,
            rate_limit_window_secs,
        })
    }
}

impl AppConfig {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn from_env() -> Result<Self> {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port_str = env::var("APP_PORT").unwrap_or_else(|_| "4000".to_string());
        let port = port_str
            .parse::<u16>()
            .context("APP_PORT must be a valid u16")?;

        let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

        let auth = AuthConfig::from_env()?;
        let mangadex = MangaDexConfig::from_env()?;

        Ok(Self {
            host,
            port,
            database_url,
            auth,
            mangadex,
        })
    }
}

#[derive(Clone)]
pub struct MangaDexConfig {
    pub base_url: String,
    pub rate_limit_per_sec: u32,
    pub cache_manga_ttl_hours: i64,
    pub cache_chapter_ttl_hours: i64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

impl MangaDexConfig {
    pub fn from_env() -> Result<Self> {
        let base_url = env::var("MANGADEX_BASE_URL")
            .unwrap_or_else(|_| "https://api.mangadex.org".to_string());

        let rate_limit_per_sec = env::var("MANGADEX_RATE_LIMIT_PER_SEC")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()
            .context("MANGADEX_RATE_LIMIT_PER_SEC must be a valid u32")?;

        let cache_manga_ttl_hours = env::var("CACHE_MANGA_TTL_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse::<i64>()
            .context("CACHE_MANGA_TTL_HOURS must be a valid i64")?;

        let cache_chapter_ttl_hours = env::var("CACHE_CHAPTER_TTL_HOURS")
            .unwrap_or_else(|_| "6".to_string())
            .parse::<i64>()
            .context("CACHE_CHAPTER_TTL_HOURS must be a valid i64")?;

        Ok(Self {
            base_url,
            rate_limit_per_sec,
            cache_manga_ttl_hours,
            cache_chapter_ttl_hours,
            username: env::var("MANGADEX_USERNAME").ok(),
            password: env::var("MANGADEX_PASSWORD").ok(),
            client_id: env::var("MANGADEX_CLIENT_ID").ok(),
            client_secret: env::var("MANGADEX_CLIENT_SECRET").ok(),
        })
    }
}
