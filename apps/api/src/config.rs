use std::env;
use anyhow::{Context, Result};

pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
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

            let database_url =
                env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

            Ok(Self {
                host,
                port,
                database_url,
            })
        }
}
