use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MangaDexResponse<T> {
    pub result: String,
    pub data: T,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub total: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct MangaDexManga {
    pub id: String,
    #[serde(rename = "type")]
    pub manga_type: String,
    pub attributes: MangaAttributes,
    pub relationships: Vec<MangaDexRelationship>,
}

#[derive(Debug, Deserialize)]
pub struct MangaAttributes {
    pub title: MangaTitle,
    #[serde(default)]
    pub alt_titles: Vec<MangaTitle>,
    #[serde(default)]
    pub description: MangaDescription,
    #[serde(rename = "originalLanguage")]
    pub original_language: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "year")]
    pub year: Option<u32>,
    #[serde(rename = "contentRating")]
    pub content_rating: String,
    #[serde(default)]
    pub tags: Vec<MangaDexTag>,
}

#[derive(Debug, Deserialize)]
pub struct MangaTitle {
    #[serde(default)]
    pub en: Option<String>,
    #[serde(default)]
    pub ja: Option<String>,
    #[serde(flatten)]
    pub other: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Default)]
pub struct MangaDescription {
    #[serde(default)]
    pub en: Option<String>,
    #[serde(default)]
    pub ja: Option<String>,
    #[serde(flatten)]
    pub other: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct MangaDexTag {
    pub id: String,
    #[serde(rename = "type")]
    pub tag_type: String,
    pub attributes: TagAttributes,
}

#[derive(Debug, Deserialize)]
pub struct TagAttributes {
    pub name: MangaTitle,
    #[serde(rename = "group")]
    pub group: String,
}

#[derive(Debug, Deserialize)]
pub struct MangaDexRelationship {
    pub id: String,
    #[serde(rename = "type")]
    pub rel_type: String,
    pub attributes: Option<RelationshipAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipAttributes {
    pub name: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MangaDexChapter {
    pub id: String,
    #[serde(rename = "type")]
    pub chapter_type: String,
    pub attributes: ChapterAttributes,
    pub relationships: Vec<MangaDexRelationship>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterAttributes {
    pub volume: Option<String>,
    pub chapter: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "translatedLanguage")]
    pub translated_language: String,
    #[serde(rename = "externalUrl")]
    pub external_url: Option<String>,
    #[serde(rename = "publishAt")]
    pub publish_at: String,
    #[serde(rename = "readableAt")]
    pub readable_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "pages")]
    pub pages: u32,
    #[serde(rename = "version")]
    pub version: u32,
}

#[derive(Debug, Deserialize)]
pub struct ChapterAtHomeResponse {
    pub result: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub chapter: ChapterAtHomeData,
}

#[derive(Debug, Deserialize)]
pub struct ChapterAtHomeData {
    pub hash: String,
    pub data: Vec<String>,
    #[serde(rename = "dataSaver")]
    pub data_saver: Vec<String>,
}

// Internal types (simplified for our API)

#[derive(Debug, Serialize, Clone)]
pub struct Manga {
    pub mangadex_id: String,
    pub title: String,
    pub alt_titles: Vec<String>,
    pub description: String,
    pub cover_url: String,
    pub status: String,
    pub year: Option<u32>,
    pub content_rating: String,
    pub tags: Vec<Tag>,
    pub author_names: Vec<String>,
    pub artist_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub group: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Chapter {
    pub mangadex_id: String,
    pub manga_mangadex_id: String,
    pub chapter_number: Option<String>,
    pub volume: Option<String>,
    pub title: Option<String>,
    pub language: String,
    pub scanlation_group_id: Option<String>,
    pub scanlation_group_name: Option<String>,
    pub page_count: u32,
    pub published_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChapterPages {
    pub chapter_id: String,
    pub base_url: String,
    pub hash: String,
    pub pages: Vec<PageInfo>,
}

#[derive(Debug, Serialize)]
pub struct PageInfo {
    pub page_number: u32,
    pub filename: String,
    pub url: String,
    #[serde(rename = "url_data_saver")]
    pub url_data_saver: String,
}
