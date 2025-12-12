use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::mangadex::types::*;

#[derive(Debug, FromRow)]
pub struct MangaCache {
    pub id: uuid::Uuid,
    pub mangadex_id: String,
    pub title: String,
    pub alt_titles: Option<serde_json::Value>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub status: Option<String>,
    pub year: Option<i32>,
    pub content_rating: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub author_names: Option<serde_json::Value>,
    pub artist_names: Option<serde_json::Value>,
    pub cached_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct ChapterCache {
    pub id: uuid::Uuid,
    pub mangadex_id: String,
    pub manga_mangadex_id: String,
    pub chapter_number: Option<String>,
    pub volume: Option<String>,
    pub title: Option<String>,
    pub language: String,
    pub scanlation_group_id: Option<String>,
    pub scanlation_group_name: Option<String>,
    pub page_count: Option<i32>,
    pub published_at: Option<DateTime<Utc>>,
    pub cached_at: DateTime<Utc>,
}

impl From<MangaCache> for Manga {
    fn from(cache: MangaCache) -> Self {
        Manga {
            mangadex_id: cache.mangadex_id.clone(),
            title: cache.title,
            alt_titles: cache
                .alt_titles
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_default(),
            description: cache.description.unwrap_or_default(),
            cover_url: cache.cover_url.unwrap_or_default(),
            status: cache.status.unwrap_or_else(|| "unknown".to_string()),
            year: cache.year.map(|y| y as u32),
            content_rating: cache.content_rating.unwrap_or_default(),
            tags: cache
                .tags
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_default(),
            author_names: cache
                .author_names
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_default(),
            artist_names: cache
                .artist_names
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_default(),
        }
    }
}

impl From<ChapterCache> for Chapter {
    fn from(cache: ChapterCache) -> Self {
        Chapter {
            mangadex_id: cache.mangadex_id,
            manga_mangadex_id: cache.manga_mangadex_id,
            chapter_number: cache.chapter_number,
            volume: cache.volume,
            title: cache.title,
            language: cache.language,
            scanlation_group_id: cache.scanlation_group_id,
            scanlation_group_name: cache.scanlation_group_name,
            page_count: cache.page_count.unwrap_or(0) as u32,
            published_at: cache.published_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

impl TryFrom<MangaDexManga> for Manga {
    type Error = anyhow::Error;

    fn try_from(mangadex: MangaDexManga) -> Result<Self, Self::Error> {
        let attrs = &mangadex.attributes;

        let title = attrs
            .title
            .en
            .clone()
            .or_else(|| attrs.title.ja.clone())
            .or_else(|| {
                attrs
                    .title
                    .other
                    .values()
                    .next()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            })
            .unwrap_or_else(|| "Untitled".to_string());

        let alt_titles: Vec<String> = attrs
            .alt_titles
            .iter()
            .flat_map(|t| t.en.clone().or_else(|| t.ja.clone()))
            .collect();

        let description = attrs
            .description
            .en
            .clone()
            .or_else(|| attrs.description.ja.clone())
            .unwrap_or_default();

        let cover_url = mangadex
            .relationships
            .iter()
            .find(|r| r.rel_type == "cover_art")
            .and_then(|r| r.attributes.as_ref())
            .and_then(|a| a.file_name.as_ref())
            .map(|filename| {
                format!(
                    "https://uploads.mangadex.org/covers/{}/{}",
                    mangadex.id, filename
                )
            })
            .unwrap_or_default();

        let mut author_names = Vec::new();
        let mut artist_names = Vec::new();

        for rel in &mangadex.relationships {
            if let Some(attrs) = &rel.attributes {
                if let Some(name) = &attrs.name {
                    match rel.rel_type.as_str() {
                        "author" => author_names.push(name.clone()),
                        "artist" => artist_names.push(name.clone()),
                        _ => {}
                    }
                }
            }
        }

        let tags: Vec<Tag> = attrs
            .tags
            .iter()
            .map(|t| {
                let name = t
                    .attributes
                    .name
                    .en
                    .clone()
                    .or_else(|| t.attributes.name.ja.clone())
                    .unwrap_or_else(|| "Unknown".to_string());

                Tag {
                    id: t.id.clone(),
                    name,
                    group: t.attributes.group.clone(),
                }
            })
            .collect();

        Ok(Manga {
            mangadex_id: mangadex.id,
            title,
            alt_titles,
            description,
            cover_url,
            status: attrs.status.clone(),
            year: attrs.year,
            content_rating: attrs.content_rating.clone(),
            tags,
            author_names,
            artist_names,
        })
    }
}

impl TryFrom<MangaDexChapter> for Chapter {
    type Error = anyhow::Error;

    fn try_from(mangadex: MangaDexChapter) -> Result<Self, Self::Error> {
        let attrs = &mangadex.attributes;

        let manga_mangadex_id = mangadex
            .relationships
            .iter()
            .find(|r| r.rel_type == "manga")
            .map(|r| r.id.clone())
            .ok_or_else(|| anyhow::anyhow!("Chapter missing manga relationship"))?;

        let (scanlation_group_id, scanlation_group_name) = mangadex
            .relationships
            .iter()
            .find(|r| r.rel_type == "scanlation_group")
            .map(|r| {
                let name = r.attributes.as_ref().and_then(|a| a.name.clone());
                (Some(r.id.clone()), name)
            })
            .unwrap_or((None, None));

        let published_at = attrs
            .publish_at
            .parse::<DateTime<Utc>>()
            .ok()
            .map(|dt| dt.to_rfc3339());

        Ok(Chapter {
            mangadex_id: mangadex.id,
            manga_mangadex_id,
            chapter_number: attrs.chapter.clone(),
            volume: attrs.volume.clone(),
            title: attrs.title.clone(),
            language: attrs.translated_language.clone(),
            scanlation_group_id,
            scanlation_group_name,
            page_count: attrs.pages,
            published_at,
        })
    }
}
