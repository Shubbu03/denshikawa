import { apiClient } from './client';
import { ENDPOINTS } from './endpoints';
import { UserResponse, userResponseSchema } from '@/lib/schemas/auth';
import {
    BookmarksResponse,
    LibraryResponse,
    ProgressResponse,
    MangaProgressResponse,
    HistoryResponse,
    UpdateProgressRequest,
    bookmarksResponseSchema,
    libraryResponseSchema,
    progressResponseSchema,
    mangaProgressResponseSchema,
    historyResponseSchema,
    updateProgressRequestSchema,
} from '@/lib/schemas/user';

export const userApi = {
    getMe: async () => {
        const { data } = await apiClient.get<UserResponse>(ENDPOINTS.USER.ME);
        return userResponseSchema.parse(data);
    },

    getLibrary: async () => {
        const { data } = await apiClient.get<LibraryResponse>(ENDPOINTS.USER.LIBRARY);
        return libraryResponseSchema.parse(data);
    },

    getBookmarks: async () => {
        const { data } = await apiClient.get<BookmarksResponse>(ENDPOINTS.USER.BOOKMARKS);
        return bookmarksResponseSchema.parse(data);
    },

    addBookmark: async (mangaId: string) => {
        await apiClient.post(ENDPOINTS.USER.BOOKMARK(mangaId));
    },

    removeBookmark: async (mangaId: string) => {
        await apiClient.delete(ENDPOINTS.USER.BOOKMARK(mangaId));
    },

    getAllProgress: async () => {
        const { data } = await apiClient.get<ProgressResponse>(ENDPOINTS.USER.PROGRESS);
        return progressResponseSchema.parse(data);
    },

    getMangaProgress: async (mangaId: string) => {
        const { data } = await apiClient.get<MangaProgressResponse>(
            ENDPOINTS.USER.MANGA_PROGRESS(mangaId)
        );
        return mangaProgressResponseSchema.parse(data);
    },

    updateProgress: async (mangaId: string, progress: UpdateProgressRequest) => {
        const validated = updateProgressRequestSchema.parse(progress);
        await apiClient.put(ENDPOINTS.USER.MANGA_PROGRESS(mangaId), validated);
    },

    getHistory: async () => {
        const { data } = await apiClient.get<HistoryResponse>(ENDPOINTS.USER.HISTORY);
        return historyResponseSchema.parse(data);
    },

    markChapterRead: async (chapterId: string) => {
        await apiClient.post(ENDPOINTS.USER.MARK_READ(chapterId));
    },

    removeFromHistory: async (chapterId: string) => {
        await apiClient.delete(ENDPOINTS.USER.MARK_READ(chapterId));
    },
};

