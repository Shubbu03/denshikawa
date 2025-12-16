use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::mangadex::{cache::get_chapters_with_cache, MangaDexError};
use crate::manga::models::ChapterCache;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ChaptersQuery {
    #[serde(default)]
    pub lang: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    100
}

#[derive(serde::Serialize)]
pub struct ChaptersResponse {
    pub data: Vec<crate::manga::Chapter>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

pub async fn get_chapters(
    Path(mangadex_id): Path<String>,
    Query(params): Query<ChaptersQuery>,
    State(state): State<AppState>,
) -> Result<Json<ChaptersResponse>, MangaDexError> {
    let limit = params.limit.min(500); // Max 500 per request

    // Get cached chapters with pagination (filter by language if provided)
    let (cached, total) = if let Some(ref lang) = params.lang {
        let cached_chapters = sqlx::query_as::<_, ChapterCache>(
            r#"SELECT id, mangadex_id, manga_mangadex_id, chapter_number, volume, title, language, scanlation_group_id, scanlation_group_name, page_count, published_at, cached_at FROM chapter_cache WHERE manga_mangadex_id = $1 AND language = $2 ORDER BY (CASE WHEN chapter_number ~ '^[0-9]+\.?[0-9]*$' THEN chapter_number::numeric ELSE 0 END) DESC, chapter_number DESC LIMIT $3 OFFSET $4"#
        )
        .bind(&mangadex_id)
        .bind(lang)
        .bind(limit as i64)
        .bind(params.offset as i64)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| MangaDexError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM chapter_cache WHERE manga_mangadex_id = $1 AND language = $2"
        )
        .bind(&mangadex_id)
        .bind(lang)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| MangaDexError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        (cached_chapters, total)
    } else {
        // No language filter - get all languages
        let cached_chapters = sqlx::query_as::<_, ChapterCache>(
            r#"SELECT id, mangadex_id, manga_mangadex_id, chapter_number, volume, title, language, scanlation_group_id, scanlation_group_name, page_count, published_at, cached_at FROM chapter_cache WHERE manga_mangadex_id = $1 ORDER BY (CASE WHEN chapter_number ~ '^[0-9]+\.?[0-9]*$' THEN chapter_number::numeric ELSE 0 END) DESC, chapter_number DESC LIMIT $2 OFFSET $3"#
        )
        .bind(&mangadex_id)
        .bind(limit as i64)
        .bind(params.offset as i64)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| MangaDexError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM chapter_cache WHERE manga_mangadex_id = $1"
        )
        .bind(&mangadex_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| MangaDexError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        (cached_chapters, total)
    };

    let chapters: Vec<crate::manga::Chapter> = cached.into_iter().map(|c| c.into()).collect();

    if chapters.is_empty() || (total == 0) || (params.offset + limit > total as u32) {
        // Fetch all chapters from MangaDex and cache them
        let lang_ref = params.lang.as_deref();
        let all_chapters = get_chapters_with_cache(
            &mangadex_id,
            lang_ref,
            &state.db_pool,
            &state.mangadex_client,
            &state.mangadex_config,
        )
        .await?;

        let mut sorted_chapters = all_chapters;
        sorted_chapters.sort_by(|a, b| {
            let num_a = a.chapter_number.as_ref()
                .and_then(|n| n.parse::<f64>().ok())
                .unwrap_or(0.0);
            let num_b = b.chapter_number.as_ref()
                .and_then(|n| n.parse::<f64>().ok())
                .unwrap_or(0.0);
            num_b.partial_cmp(&num_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        let total = sorted_chapters.len() as u32;
        let start = params.offset as usize;
        let end = (start + limit as usize).min(sorted_chapters.len());
        let paginated = sorted_chapters.into_iter().skip(start).take(end - start).collect();

        return Ok(Json(ChaptersResponse {
            data: paginated,
            total,
            limit,
            offset: params.offset,
        }));
    }

    Ok(Json(ChaptersResponse {
        data: chapters,
        total: total as u32,
        limit,
        offset: params.offset,
    }))
}

