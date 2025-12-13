import { apiClient } from './client';
import { ENDPOINTS } from './endpoints';
import {
    MangaSearchResponse,
    MangaDetails,
    mangaSearchSchema,
    mangaDetailsSchema,
} from '@/lib/schemas/manga';
import { ChapterList, chapterListSchema } from '@/lib/schemas/chapter';

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

    getDetails: async (id: string) => {
        const { data } = await apiClient.get<MangaDetails>(ENDPOINTS.MANGA.DETAILS(id));
        return mangaDetailsSchema.parse(data);
    },

    getChapters: async (id: string, lang = 'en') => {
        const { data } = await apiClient.get<ChapterList>(ENDPOINTS.MANGA.CHAPTERS(id), {
            params: { lang },
        });
        return chapterListSchema.parse(data);
    },
};

