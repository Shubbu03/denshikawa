import { z } from 'zod';

export const chapterSchema = z.object({
    mangadex_id: z.string(),
    manga_mangadex_id: z.string(),
    chapter_number: z.string().nullable(),
    volume: z.string().nullable(),
    title: z.string().nullable(),
    language: z.string(),
    scanlation_group_name: z.string().nullable(),
    page_count: z.number().nullable(),
    published_at: z.string().nullable(),
});

export const chapterListSchema = z.array(chapterSchema);

export const chapterListResponseSchema = z.object({
    data: z.array(chapterSchema),
    total: z.number(),
    limit: z.number(),
    offset: z.number(),
});

export const chapterPagesSchema = z.object({
    chapter_id: z.string(),
    base_url: z.string(),
    hash: z.string(),
    pages: z.array(
        z.object({
            page_number: z.number(),
            filename: z.string(),
            url: z.string(),
            url_data_saver: z.string(),
        })
    ),
});

export const chapterNavigationSchema = z.object({
    prev_chapter_id: z.string().nullable(),
    next_chapter_id: z.string().nullable(),
    current_chapter_id: z.string(),
});

export type Chapter = z.infer<typeof chapterSchema>;
export type ChapterList = z.infer<typeof chapterListSchema>;
export type ChapterListResponse = z.infer<typeof chapterListResponseSchema>;
export type ChapterPages = z.infer<typeof chapterPagesSchema>;
export type ChapterNavigation = z.infer<typeof chapterNavigationSchema>;

