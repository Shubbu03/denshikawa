use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::config::MangaDexConfig;
use crate::manga::models::{ChapterCache, MangaCache};
use crate::mangadex::client::MangaDexClient;
use crate::mangadex::error::MangaDexError;
use crate::mangadex::types::*;

pub async fn get_manga_with_cache(
    mangadex_id: &str,
    db: &PgPool,
    client: &MangaDexClient,
    config: &MangaDexConfig,
) -> Result<Manga, MangaDexError> {
    let cached = sqlx::query_as::<_, MangaCache>(
        "SELECT id, mangadex_id, title, alt_titles, description, cover_url, status::text AS status, year, content_rating, tags, author_names, artist_names, cached_at FROM manga_cache WHERE mangadex_id = $1"
    )
    .bind(mangadex_id)
    .fetch_optional(db)
    .await
    .map_err(|e| MangaDexError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    if let Some(manga_cache) = cached {
        let cache_age = Utc::now() - manga_cache.cached_at;
        if cache_age < Duration::hours(config.cache_manga_ttl_hours) {
            return Ok(manga_cache.into());
        }
    }

    let mangadex_manga = client.get_manga(mangadex_id).await?;
    let manga: Manga = mangadex_manga.try_into()?;

    sqlx::query(
        r#"
        INSERT INTO manga_cache (
            mangadex_id, title, alt_titles, description, cover_url,
            status, year, content_rating, tags, author_names, artist_names
        )
        VALUES ($1, $2, $3, $4, $5, $6::manga_status, $7, $8, $9, $10, $11)
        ON CONFLICT (mangadex_id) DO UPDATE SET
            title = EXCLUDED.title,
            alt_titles = EXCLUDED.alt_titles,
            description = EXCLUDED.description,
            cover_url = EXCLUDED.cover_url,
            status = EXCLUDED.status,
            year = EXCLUDED.year,
            content_rating = EXCLUDED.content_rating,
            tags = EXCLUDED.tags,
            author_names = EXCLUDED.author_names,
            artist_names = EXCLUDED.artist_names,
            cached_at = NOW()
        "#,
    )
    .bind(&manga.mangadex_id)
    .bind(&manga.title)
    .bind(serde_json::to_value(&manga.alt_titles).ok())
    .bind(&manga.description)
    .bind(&manga.cover_url)
    .bind(&manga.status)
    .bind(manga.year.map(|y| y as i32))
    .bind(&manga.content_rating)
    .bind(serde_json::to_value(&manga.tags).ok())
    .bind(serde_json::to_value(&manga.author_names).ok())
    .bind(serde_json::to_value(&manga.artist_names).ok())
    .execute(db)
    .await
    .map_err(|e| MangaDexError::Internal(anyhow::anyhow!("Failed to cache manga: {}", e)))?;

    Ok(manga)
}

pub async fn get_chapters_with_cache(
    manga_mangadex_id: &str,
    lang: &str,
    db: &PgPool,
    client: &MangaDexClient,
    config: &MangaDexConfig,
) -> Result<Vec<Chapter>, MangaDexError> {
    let cached = sqlx::query_as::<_, ChapterCache>(
        "SELECT id, mangadex_id, manga_mangadex_id, chapter_number, volume, title, language, scanlation_group_id, scanlation_group_name, page_count, published_at, cached_at FROM chapter_cache WHERE manga_mangadex_id = $1 AND language = $2 ORDER BY chapter_number::numeric"
    )
    .bind(manga_mangadex_id)
    .bind(lang)
    .fetch_all(db)
    .await
    .map_err(|e| MangaDexError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    if !cached.is_empty() {
        let oldest_cache = cached.iter().map(|c| c.cached_at).min().unwrap();
        let cache_age = Utc::now() - oldest_cache;
        if cache_age < Duration::hours(config.cache_chapter_ttl_hours) {
            return Ok(cached.into_iter().map(|c| c.into()).collect());
        }
    }

    let mut all_chapters = Vec::new();
    let mut offset = 0;
    let limit = 100;

    loop {
        let response = client
            .get_chapters(manga_mangadex_id, lang, limit, offset)
            .await?;

        if response.data.is_empty() {
            break;
        }

        for mangadex_chapter in response.data {
            let chapter: Chapter = mangadex_chapter.try_into()?;
            all_chapters.push(chapter.clone());

            sqlx::query(
                r#"
                INSERT INTO chapter_cache (
                    mangadex_id, manga_mangadex_id, chapter_number, volume,
                    title, language, scanlation_group_id, scanlation_group_name,
                    page_count, published_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                ON CONFLICT (mangadex_id) DO UPDATE SET
                    chapter_number = EXCLUDED.chapter_number,
                    volume = EXCLUDED.volume,
                    title = EXCLUDED.title,
                    language = EXCLUDED.language,
                    scanlation_group_id = EXCLUDED.scanlation_group_id,
                    scanlation_group_name = EXCLUDED.scanlation_group_name,
                    page_count = EXCLUDED.page_count,
                    published_at = EXCLUDED.published_at,
                    cached_at = NOW()
                "#,
            )
            .bind(&chapter.mangadex_id)
            .bind(&chapter.manga_mangadex_id)
            .bind(&chapter.chapter_number)
            .bind(&chapter.volume)
            .bind(&chapter.title)
            .bind(&chapter.language)
            .bind(&chapter.scanlation_group_id)
            .bind(&chapter.scanlation_group_name)
            .bind(chapter.page_count as i32)
            .bind(chapter.published_at.as_ref().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.with_timezone(&chrono::Utc))
            }))
            .execute(db)
            .await
            .map_err(|e| {
                MangaDexError::Internal(anyhow::anyhow!("Failed to cache chapter: {}", e))
            })?;
        }

        offset += limit;
        if let Some(total) = response.total {
            if offset >= total {
                break;
            }
        } else {
            break;
        }
    }

    Ok(all_chapters)
}
