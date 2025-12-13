import { apiClient } from './client';
import { ENDPOINTS } from './endpoints';
import {
    ChapterPages,
    ChapterNavigation,
    chapterPagesSchema,
    chapterNavigationSchema,
} from '@/lib/schemas/chapter';

export const chaptersApi = {
    getPages: async (id: string) => {
        const { data } = await apiClient.get<ChapterPages>(ENDPOINTS.CHAPTERS.PAGES(id));
        return chapterPagesSchema.parse(data);
    },

    getNavigation: async (id: string) => {
        const { data } = await apiClient.get<ChapterNavigation>(
            ENDPOINTS.CHAPTERS.NAVIGATION(id)
        );
        return chapterNavigationSchema.parse(data);
    },
};

