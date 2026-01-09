use axum::{
    extract::{Path, State},
    Json,
};

use crate::mangadex::MangaDexError;
use crate::mangadex::types::ChapterAtHomeResponse;
use crate::AppState;

#[derive(serde::Serialize)]
pub struct ChapterPagesResponse {
    pub chapter_id: String,
    pub base_url: String,
    pub hash: String,
    pub pages: Vec<PageInfoResponse>,
}

#[derive(serde::Serialize)]
pub struct PageInfoResponse {
    pub page_number: u32,
    pub filename: String,
    pub url: String,
    pub url_data_saver: String,
}

pub async fn get_chapter_pages(
    Path(chapter_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ChapterPagesResponse>, MangaDexError> {
    let at_home_data = state
        .mangadex_client
        .get_chapter_pages(&chapter_id)
        .await?;

    let pages: Vec<PageInfoResponse> = at_home_data
        .chapter
        .data
        .iter()
        .enumerate()
        .map(|(idx, filename)| {
            let page_number = idx + 1;
            let quality_mode = "data";
            let data_saver_mode = "data-saver";
            let url = format!(
                "{}/{}/{}/{}",
                at_home_data.base_url, quality_mode, at_home_data.chapter.hash, filename
            );
            let url_data_saver = if let Some(data_saver_filename) = at_home_data.chapter.data_saver.get(idx) {
                format!(
                    "{}/{}/{}/{}",
                    at_home_data.base_url, data_saver_mode, at_home_data.chapter.hash, data_saver_filename
                )
            } else {
                url.clone()
            };

            PageInfoResponse {
                page_number: page_number as u32,
                filename: filename.clone(),
                url,
                url_data_saver,
            }
        })
        .collect();

    Ok(Json(ChapterPagesResponse {
        chapter_id: chapter_id.clone(),
        base_url: at_home_data.base_url,
        hash: at_home_data.chapter.hash,
        pages,
    }))
}
