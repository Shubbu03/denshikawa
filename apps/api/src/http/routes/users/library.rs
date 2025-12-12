use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use sqlx::FromRow;

use crate::auth::CurrentUser;
use crate::mangadex::cache::get_manga_with_cache;
use crate::AppState;

#[derive(Serialize, FromRow)]
struct LibraryManga {
    manga_mangadex_id: String,
    bookmark_created_at: String,
    chapter_mangadex_id: Option<String>,
    page_number: Option<i32>,
    progress_updated_at: Option<String>,
}

#[derive(Serialize)]
pub struct LibraryItem {
    pub manga: crate::manga::Manga,
    pub progress: Option<LibraryProgress>,
    pub bookmarked_at: String,
}

#[derive(Serialize)]
pub struct LibraryProgress {
    pub chapter_id: String,
    pub page_number: u32,
    pub updated_at: String,
}

pub async fn get_library(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<Vec<LibraryItem>>, StatusCode> {
    let library_data = sqlx::query_as::<_, LibraryManga>(
        r#"
        SELECT 
            b.manga_mangadex_id,
            b.created_at::text AS bookmark_created_at,
            p.chapter_mangadex_id,
            p.page_number,
            p.updated_at::text AS progress_updated_at
        FROM user_bookmarks b
        LEFT JOIN user_reading_progress p 
            ON b.user_id = p.user_id 
            AND b.manga_mangadex_id = p.manga_mangadex_id
        WHERE b.user_id = $1
        ORDER BY COALESCE(p.updated_at, b.created_at) DESC
        "#,
    )
    .bind(user.id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch manga details for each bookmark
    let mut library_items = Vec::new();
    for item in library_data {
        let manga = get_manga_with_cache(
            &item.manga_mangadex_id,
            &state.db_pool,
            &state.mangadex_client,
            &state.mangadex_config,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let progress = if let (Some(chapter_id), Some(page_number)) =
            (item.chapter_mangadex_id, item.page_number)
        {
            Some(LibraryProgress {
                chapter_id,
                page_number: page_number as u32,
                updated_at: item.progress_updated_at.unwrap_or_default(),
            })
        } else {
            None
        };

        library_items.push(LibraryItem {
            manga,
            progress,
            bookmarked_at: item.bookmark_created_at,
        });
    }

    Ok(Json(library_items))
}
