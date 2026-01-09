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
        RANDOM: '/manga/random',
        DETAILS: (id: string) => `/manga/${id}`,
        CHAPTERS: (id: string) => `/manga/${id}/chapters`,
        AGGREGATE: (id: string) => `/manga/${id}/aggregate`,
        CHAPTER_PAGES: (chapterId: string) => `/manga/chapter/${chapterId}/pages`,
    },

    // User
    USER: {
        ME: '/users/me',
    },

    // Proxy
    PROXY: {
        IMAGE: '/proxy/image',
    },
} as const;

