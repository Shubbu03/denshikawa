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

export const chapterPagesSchema = z.object({
    pages: z.array(
        z.object({
            url: z.string(),
            width: z.number(),
            height: z.number(),
            page_number: z.number(),
        })
    ),
});

export const chapterNavigationSchema = z.object({
    previous: z
        .object({
            mangadex_id: z.string(),
            chapter_number: z.string().nullable(),
        })
        .nullable(),
    next: z
        .object({
            mangadex_id: z.string(),
            chapter_number: z.string().nullable(),
        })
        .nullable(),
});

export type Chapter = z.infer<typeof chapterSchema>;
export type ChapterList = z.infer<typeof chapterListSchema>;
export type ChapterPages = z.infer<typeof chapterPagesSchema>;
export type ChapterNavigation = z.infer<typeof chapterNavigationSchema>;

