export const ENDPOINTS = {
    // Auth
    AUTH: {
        LOGIN: '/auth/login',
        REGISTER: '/auth/register',
        REFRESH: '/auth/refresh',
        LOGOUT: '/auth/logout',
        GOOGLE: '/auth/google',
    },

    // Manga
    MANGA: {
        SEARCH: '/manga/search',
        POPULAR: '/manga/popular',
        LATEST: '/manga/latest',
        DETAILS: (id: string) => `/manga/${id}`,
        CHAPTERS: (id: string) => `/manga/${id}/chapters`,
    },

    // Chapters
    CHAPTERS: {
        PAGES: (id: string) => `/chapters/${id}/pages`,
        NAVIGATION: (id: string) => `/chapters/${id}/navigation`,
    },

    // User
    USER: {
        ME: '/users/me',
        LIBRARY: '/users/me/library',
        BOOKMARKS: '/users/me/bookmarks',
        BOOKMARK: (mangaId: string) => `/users/me/bookmarks/${mangaId}`,
        PROGRESS: '/users/me/progress',
        MANGA_PROGRESS: (mangaId: string) => `/users/me/progress/${mangaId}`,
        HISTORY: '/users/me/history',
        MARK_READ: (chapterId: string) => `/users/me/history/${chapterId}`,
    },

    // Proxy
    PROXY: {
        IMAGE: '/proxy/image',
    },
} as const;

