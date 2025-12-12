use axum::{
    extract::{Path, State},
    Json,
};

use crate::mangadex::{types::*, MangaDexError};
use crate::AppState;

#[derive(serde::Serialize)]
pub struct ChapterPagesResponse {
    pub chapter_id: String,
    pub base_url: String,
    pub hash: String,
    pub pages: Vec<PageInfo>,
}

pub async fn get_chapter_pages(
    Path(chapter_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ChapterPagesResponse>, MangaDexError> {
    let chapter = state.mangadex_client.get_chapter(&chapter_id).await?;

    if chapter.attributes.external_url.is_some() {
        return Err(MangaDexError::ApiError(
            "This chapter is hosted externally and does not have pages available".to_string(),
        ));
    }

    // Check if chapter has pages
    if chapter.attributes.pages == 0 {
        return Err(MangaDexError::ApiError(
            "This chapter has no pages available".to_string(),
        ));
    }

    let response = state.mangadex_client.get_chapter_pages(&chapter_id).await?;

    if response.chapter.data.is_empty() && response.chapter.data_saver.is_empty() {
        return Err(MangaDexError::ApiError(format!(
            "Chapter has {} pages but no image data available",
            chapter.attributes.pages
        )));
    }

    let page_filenames = if !response.chapter.data.is_empty() {
        &response.chapter.data
    } else {
        &response.chapter.data_saver
    };

    let pages: Vec<PageInfo> = page_filenames
        .iter()
        .enumerate()
        .map(|(idx, filename)| {
            let page_number = idx + 1;

            let (url, url_data_saver) = if !response.chapter.data.is_empty() {
                let url = format!(
                    "{}/data/{}/{}",
                    response.base_url, response.chapter.hash, filename
                );
                let url_data_saver = response
                    .chapter
                    .data_saver
                    .get(idx)
                    .map(|ds_filename| {
                        format!(
                            "{}/data-saver/{}/{}",
                            response.base_url, response.chapter.hash, ds_filename
                        )
                    })
                    .unwrap_or_else(|| url.clone());
                (url, url_data_saver)
            } else {
                let url_data_saver = format!(
                    "{}/data-saver/{}/{}",
                    response.base_url, response.chapter.hash, filename
                );
                (url_data_saver.clone(), url_data_saver)
            };

            PageInfo {
                page_number: page_number as u32,
                filename: filename.clone(),
                url,
                url_data_saver,
            }
        })
        .collect();

    Ok(Json(ChapterPagesResponse {
        chapter_id,
        base_url: response.base_url,
        hash: response.chapter.hash,
        pages,
    }))
}
