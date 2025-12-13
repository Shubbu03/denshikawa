import { z } from 'zod';

export const bookmarkSchema = z.object({
    manga_id: z.string(),
    mangadex_id: z.string(),
    title: z.string(),
    cover_url: z.string(),
    added_at: z.string(),
});

export const bookmarksResponseSchema = z.array(bookmarkSchema);

export const progressSchema = z.object({
    manga_id: z.string(),
    mangadex_id: z.string(),
    current_chapter_id: z.string().nullable(),
    current_page: z.number().nullable(),
    updated_at: z.string(),
});

export const progressResponseSchema = z.array(progressSchema);

export const mangaProgressResponseSchema = z.object({
    manga_id: z.string(),
    mangadex_id: z.string(),
    current_chapter_id: z.string().nullable(),
    current_page: z.number().nullable(),
    updated_at: z.string(),
});

export const updateProgressRequestSchema = z.object({
    chapter_id: z.string(),
    page: z.number().min(0),
});

export const historyItemSchema = z.object({
    chapter_id: z.string(),
    mangadex_id: z.string(),
    manga_mangadex_id: z.string(),
    chapter_number: z.string().nullable(),
    title: z.string().nullable(),
    read_at: z.string(),
});

export const historyResponseSchema = z.array(historyItemSchema);

export const libraryItemSchema = z.object({
    manga_id: z.string(),
    mangadex_id: z.string(),
    title: z.string(),
    cover_url: z.string(),
    status: z.string(),
    current_chapter_id: z.string().nullable(),
    current_page: z.number().nullable(),
    last_read_at: z.string().nullable(),
    added_at: z.string(),
});

export const libraryResponseSchema = z.array(libraryItemSchema);

export type Bookmark = z.infer<typeof bookmarkSchema>;
export type BookmarksResponse = z.infer<typeof bookmarksResponseSchema>;
export type Progress = z.infer<typeof progressSchema>;
export type ProgressResponse = z.infer<typeof progressResponseSchema>;
export type MangaProgressResponse = z.infer<typeof mangaProgressResponseSchema>;
export type UpdateProgressRequest = z.infer<typeof updateProgressRequestSchema>;
export type HistoryItem = z.infer<typeof historyItemSchema>;
export type HistoryResponse = z.infer<typeof historyResponseSchema>;
export type LibraryItem = z.infer<typeof libraryItemSchema>;
export type LibraryResponse = z.infer<typeof libraryResponseSchema>;

