use backoff::ExponentialBackoff;
use governor::{Quota, RateLimiter};
use reqwest::Client;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

use crate::config::MangaDexConfig;
use crate::mangadex::error::MangaDexError;
use crate::mangadex::types::*;

#[derive(Clone)]
struct TokenPair {
    access_token: String,
    refresh_token: String,
    expires_at: Instant,
}

pub struct MangaDexClient {
    http: Client,
    rate_limiter: Arc<
        RateLimiter<
            governor::state::direct::NotKeyed,
            governor::state::InMemoryState,
            governor::clock::DefaultClock,
        >,
    >,
    base_url: String,
    auth_url: String,
    tokens: Arc<Mutex<Option<TokenPair>>>,
    config: MangaDexConfig,
}

impl MangaDexClient {
    pub fn new(config: &MangaDexConfig) -> Result<Self, MangaDexError> {
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| {
                MangaDexError::Internal(anyhow::anyhow!("Failed to create HTTP client: {}", e))
            })?;

        let quota = Quota::per_second(
            NonZeroU32::new(config.rate_limit_per_sec)
                .ok_or_else(|| MangaDexError::Internal(anyhow::anyhow!("Invalid rate limit")))?,
        );
        let rate_limiter = Arc::new(RateLimiter::direct(quota));

        Ok(Self {
            http,
            rate_limiter,
            base_url: config.base_url.clone(),
            auth_url: "https://auth.mangadex.org/realms/mangadex/protocol/openid-connect/token".to_string(),
            tokens: Arc::new(Mutex::new(None)),
            config: config.clone(),
        })
    }

    async fn authenticate(&self) -> Result<(), MangaDexError> {
        let username = self.config.username.as_ref()
            .ok_or_else(|| MangaDexError::ApiError("MANGADEX_USERNAME not set".to_string()))?;
        let password = self.config.password.as_ref()
            .ok_or_else(|| MangaDexError::ApiError("MANGADEX_PASSWORD not set".to_string()))?;
        let client_id = self.config.client_id.as_ref()
            .ok_or_else(|| MangaDexError::ApiError("MANGADEX_CLIENT_ID not set".to_string()))?;
        let client_secret = self.config.client_secret.as_ref()
            .ok_or_else(|| MangaDexError::ApiError("MANGADEX_CLIENT_SECRET not set".to_string()))?;

        let form_data = [
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ];

        let response = self.http
            .post(&self.auth_url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .form(&form_data)
            .send()
            .await
            .map_err(|e| MangaDexError::NetworkError(e))?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(MangaDexError::ApiError(format!("Authentication failed: HTTP {} - {}", status, text)));
        }

        #[derive(serde::Deserialize)]
        struct AuthResponse {
            access_token: String,
            refresh_token: String,
        }

        let auth: AuthResponse = response.json().await
            .map_err(|_| MangaDexError::InvalidResponse)?;

        let tokens = TokenPair {
            access_token: auth.access_token,
            refresh_token: auth.refresh_token,
            expires_at: Instant::now() + Duration::from_secs(14 * 60),
        };

        *self.tokens.lock().await = Some(tokens);
        Ok(())
    }

    async fn refresh_token(&self) -> Result<(), MangaDexError> {
        let refresh_token = {
            let tokens = self.tokens.lock().await;
            tokens.as_ref()
                .map(|t| t.refresh_token.clone())
                .ok_or_else(|| MangaDexError::ApiError("No refresh token available".to_string()))?
        };

        let client_id = self.config.client_id.as_ref()
            .ok_or_else(|| MangaDexError::ApiError("MANGADEX_CLIENT_ID not set".to_string()))?;
        let client_secret = self.config.client_secret.as_ref()
            .ok_or_else(|| MangaDexError::ApiError("MANGADEX_CLIENT_SECRET not set".to_string()))?;

        let form_data = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &refresh_token),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ];

        let response = self.http
            .post(&self.auth_url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .form(&form_data)
            .send()
            .await
            .map_err(|e| MangaDexError::NetworkError(e))?;

        let status = response.status();
        if !status.is_success() {
            let _text = response.text().await.unwrap_or_default();
            *self.tokens.lock().await = None;
            return self.authenticate().await;
        }

        #[derive(serde::Deserialize)]
        struct RefreshResponse {
            access_token: String,
            #[serde(default)]
            refresh_token: Option<String>,
        }

        let refresh: RefreshResponse = response.json().await
            .map_err(|_| MangaDexError::InvalidResponse)?;

        let mut tokens = self.tokens.lock().await;
        if let Some(existing) = tokens.as_mut() {
            existing.access_token = refresh.access_token;
            existing.expires_at = Instant::now() + Duration::from_secs(14 * 60);
            if let Some(new_refresh) = refresh.refresh_token {
                existing.refresh_token = new_refresh;
            }
        } else {
            return Err(MangaDexError::ApiError("Token state inconsistent".to_string()));
        }

        Ok(())
    }

    async fn ensure_authenticated(&self) -> Result<(), MangaDexError> {
        if self.config.username.is_none() || self.config.password.is_none() 
            || self.config.client_id.is_none() || self.config.client_secret.is_none() {
            return Ok(());
        }

        let needs_auth = {
            let tokens = self.tokens.lock().await;
            tokens.is_none() || tokens.as_ref().unwrap().expires_at <= Instant::now()
        };

        if needs_auth {
            let tokens = self.tokens.lock().await;
            if tokens.is_none() {
                drop(tokens);
                self.authenticate().await?;
            } else {
                drop(tokens);
                self.refresh_token().await?;
            }
        }

        Ok(())
    }

    async fn request_with_retry<F, Fut, T>(&self, mut op: F) -> Result<T, MangaDexError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, backoff::Error<MangaDexError>>>,
    {
        let backoff_config = ExponentialBackoff {
            max_elapsed_time: Some(Duration::from_secs(30)),
            initial_interval: Duration::from_millis(100),
            max_interval: Duration::from_secs(5),
            multiplier: 2.0,
            ..Default::default()
        };

        let start = std::time::Instant::now();
        let mut attempt = 0u32;
        loop {
            match op().await {
                Ok(result) => return Ok(result),
                Err(backoff::Error::Permanent(err)) => return Err(err),
                Err(backoff::Error::Transient {
                    err,
                    retry_after,
                }) => {
                    if let Some(max_time) = backoff_config.max_elapsed_time {
                        if start.elapsed() >= max_time {
                            return Err(err);
                        }
                    }
                    
                    let delay = if let Some(retry_duration) = retry_after {
                        retry_duration
                    } else {
                        let base_delay = backoff_config.initial_interval.as_millis() as u64;
                        let delay_ms =
                            (base_delay as f64 * backoff_config.multiplier.powi(attempt as i32)) as u64;
                        Duration::from_millis(
                            delay_ms.min(backoff_config.max_interval.as_millis() as u64),
                        )
                    };
                    attempt += 1;
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    async fn get_json<T>(&self, url: &str) -> Result<T, backoff::Error<MangaDexError>>
    where
        T: serde::de::DeserializeOwned,
    {
        if let Err(e) = self.ensure_authenticated().await {
            return Err(backoff::Error::Permanent(e));
        }

        while self.rate_limiter.check().is_err() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let access_token = {
            let tokens = self.tokens.lock().await;
            tokens.as_ref().map(|t| t.access_token.clone())
        };

        let mut request = self
            .http
            .get(url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .header("Accept", "application/json");

        if let Some(token) = access_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    backoff::Error::Transient {
                        err: MangaDexError::NetworkError(e),
                        retry_after: None,
                    }
                } else {
                    backoff::Error::Permanent(MangaDexError::NetworkError(e))
                }
            })?;

        let status = response.status();

        if status == 401 {
            if let Err(_) = self.refresh_token().await {
                if let Err(_) = self.authenticate().await {
                    return Err(backoff::Error::Transient {
                        err: MangaDexError::ApiError("Authentication failed".to_string()),
                        retry_after: Some(Duration::from_secs(5)),
                    });
                }
            }
            return Err(backoff::Error::Transient {
                err: MangaDexError::ApiError("Token expired, retrying".to_string()),
                retry_after: None,
            });
        }

        if status == 429 {
            return Err(backoff::Error::Transient {
                err: MangaDexError::RateLimited,
                retry_after: None,
            });
        }

        if status == 403 {
            return Err(backoff::Error::Transient {
                err: MangaDexError::ApiError("Temporarily banned by MangaDex".to_string()),
                retry_after: Some(Duration::from_secs(120)),
            });
        }

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(backoff::Error::Permanent(MangaDexError::ApiError(format!(
                "HTTP {}: {}",
                status, text
            ))));
        }

        let json: T = response
            .json()
            .await
            .map_err(|_| backoff::Error::Permanent(MangaDexError::InvalidResponse))?;

        Ok(json)
    }

    pub async fn search_manga(
        &self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> Result<MangaDexResponse<Vec<MangaDexManga>>, MangaDexError> {
        let url = format!(
            "{}/manga?title={}&limit={}&offset={}&includes[]=cover_art&includes[]=author&includes[]=artist",
            self.base_url,
            urlencoding::encode(query),
            limit,
            offset
        );

        self.request_with_retry(|| async { self.get_json(&url).await })
            .await
    }

    pub async fn get_popular_manga(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<MangaDexResponse<Vec<MangaDexManga>>, MangaDexError> {
        let url = format!(
            "{}/manga?order[followedCount]=desc&limit={}&offset={}&includes[]=cover_art&includes[]=author&includes[]=artist",
            self.base_url,
            limit,
            offset
        );

        self.request_with_retry(|| async { self.get_json(&url).await })
            .await
    }

    pub async fn get_latest_manga(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<MangaDexResponse<Vec<MangaDexManga>>, MangaDexError> {
        let url = format!(
            "{}/manga?order[latestUploadedChapter]=desc&limit={}&offset={}&includes[]=cover_art&includes[]=author&includes[]=artist",
            self.base_url,
            limit,
            offset
        );

        self.request_with_retry(|| async { self.get_json(&url).await })
            .await
    }

    pub async fn get_manga(&self, id: &str) -> Result<MangaDexManga, MangaDexError> {
        let url = format!(
            "{}/manga/{}?includes[]=cover_art&includes[]=author&includes[]=artist&includes[]=tag",
            self.base_url, id
        );

        let response: MangaDexResponse<MangaDexManga> = self
            .request_with_retry(|| async { self.get_json(&url).await })
            .await?;

        Ok(response.data)
    }

    pub async fn get_chapters(
        &self,
        manga_id: &str,
        lang: &str,
        limit: u32,
        offset: u32,
    ) -> Result<MangaDexResponse<Vec<MangaDexChapter>>, MangaDexError> {
        let url = format!(
            "{}/manga/{}/feed?translatedLanguage[]={}&limit={}&offset={}&includes[]=scanlation_group&order[chapter]=asc",
            self.base_url, manga_id, lang, limit, offset
        );

        self.request_with_retry(|| async { self.get_json(&url).await })
            .await
    }

    pub async fn get_chapter(&self, chapter_id: &str) -> Result<MangaDexChapter, MangaDexError> {
        let url = format!("{}/chapter/{}", self.base_url, chapter_id);

        let response: MangaDexResponse<MangaDexChapter> = self
            .request_with_retry(|| async { self.get_json(&url).await })
            .await?;

        Ok(response.data)
    }

    pub async fn get_chapter_pages(
        &self,
        chapter_id: &str,
    ) -> Result<ChapterAtHomeResponse, MangaDexError> {
        let url = format!("{}/at-home/server/{}", self.base_url, chapter_id);

        self.request_with_retry(|| async { self.get_json(&url).await })
            .await
    }

    pub fn get_cover_url(&self, manga_id: &str, cover_filename: &str) -> String {
        format!("{}/covers/{}/{}", self.base_url, manga_id, cover_filename)
    }
}
