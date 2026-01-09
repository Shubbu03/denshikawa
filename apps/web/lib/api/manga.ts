import { apiClient } from './client';
import { ENDPOINTS } from './endpoints';
import {
    MangaSearchResponse,
    MangaDetails,
    mangaSearchSchema,
    mangaDetailsSchema,
} from '@/lib/schemas/manga';
import { ChapterListResponse, chapterListResponseSchema, ChapterPagesResponse, chapterPagesResponseSchema } from '@/lib/schemas/chapter';

export const mangaApi = {
    search: async (query: string, limit = 20, offset = 0) => {
        const { data } = await apiClient.get<MangaSearchResponse>(ENDPOINTS.MANGA.SEARCH, {
            params: { q: query, limit, offset },
        });
        return mangaSearchSchema.parse(data);
    },

    getPopular: async (limit = 20, offset = 0) => {
        const { data } = await apiClient.get<MangaSearchResponse>(ENDPOINTS.MANGA.POPULAR, {
            params: { limit, offset },
        });
        return mangaSearchSchema.parse(data);
    },

    getLatest: async (limit = 20, offset = 0) => {
        const { data } = await apiClient.get<MangaSearchResponse>(ENDPOINTS.MANGA.LATEST, {
            params: { limit, offset },
        });
        return mangaSearchSchema.parse(data);
    },

    getRandom: async () => {
        const { data } = await apiClient.get<{ id: string }>(ENDPOINTS.MANGA.RANDOM);
        return data;
    },

    getDetails: async (id: string) => {
        const { data } = await apiClient.get<MangaDetails>(ENDPOINTS.MANGA.DETAILS(id));
        return mangaDetailsSchema.parse(data);
    },

    getChapters: async (id: string, lang: string | null = null, limit = 100, offset = 0) => {
        const params: Record<string, any> = { limit, offset };
        if (lang) {
            params.lang = lang;
        }
        const { data } = await apiClient.get<ChapterListResponse>(ENDPOINTS.MANGA.CHAPTERS(id), {
            params,
        });
        return chapterListResponseSchema.parse(data);
    },

    getAggregate: async (id: string, lang: string | null = null) => {
        const params: Record<string, any> = {};
        if (lang) {
            params.lang = lang;
        }
        const { data } = await apiClient.get(ENDPOINTS.MANGA.AGGREGATE(id), {
            params,
        });
        return data;
    },

    getChapterPages: async (chapterId: string) => {
        const { data } = await apiClient.get<ChapterPagesResponse>(ENDPOINTS.MANGA.CHAPTER_PAGES(chapterId));
        return chapterPagesResponseSchema.parse(data);
    },
};

