import { z } from 'zod';

export const mangaSummarySchema = z.object({
    id: z.string(),
    mangadex_id: z.string(),
    title: z.string(),
    cover_url: z.string(),
    status: z.string(),
});

export const mangaSearchSchema = z.object({
    data: z.array(mangaSummarySchema),
    total: z.number(),
    limit: z.number(),
    offset: z.number(),
});

export const tagSchema = z.object({
    id: z.string(),
    name: z.string(),
    group: z.string(),
});

export const mangaDetailsSchema = z.object({
    mangadex_id: z.string(),
    title: z.string(),
    alt_titles: z.array(z.string()),
    description: z.string(),
    cover_url: z.string(),
    status: z.string(),
    year: z.number().nullable(),
    content_rating: z.string(),
    tags: z.array(tagSchema),
    author_names: z.array(z.string()),
    artist_names: z.array(z.string()),
});

export type MangaSummary = z.infer<typeof mangaSummarySchema>;
export type MangaSearchResponse = z.infer<typeof mangaSearchSchema>;
export type MangaDetails = z.infer<typeof mangaDetailsSchema>;
export type Tag = z.infer<typeof tagSchema>;

