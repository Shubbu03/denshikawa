use backoff::ExponentialBackoff;
use governor::{Quota, RateLimiter};
use reqwest::Client;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use crate::config::MangaDexConfig;
use crate::mangadex::error::MangaDexError;
use crate::mangadex::types::*;

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
        })
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
                    retry_after: _,
                }) => {
                    if let Some(max_time) = backoff_config.max_elapsed_time {
                        if start.elapsed() >= max_time {
                            return Err(err);
                        }
                    }
                    // Calculate exponential backoff delay
                    let base_delay = backoff_config.initial_interval.as_millis() as u64;
                    let delay_ms =
                        (base_delay as f64 * backoff_config.multiplier.powi(attempt as i32)) as u64;
                    let delay = Duration::from_millis(
                        delay_ms.min(backoff_config.max_interval.as_millis() as u64),
                    );
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
        while self.rate_limiter.check().is_err() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let response = self
            .http
            .get(url)
            .header("User-Agent", "Denshikawa/1.0")
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
        if status == 429 {
            return Err(backoff::Error::Transient {
                err: MangaDexError::RateLimited,
                retry_after: None,
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
