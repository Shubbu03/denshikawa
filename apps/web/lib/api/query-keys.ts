export const queryKeys = {
    manga: {
        all: ['manga'] as const,
        popular: () => [...queryKeys.manga.all, 'popular'] as const,
        latest: () => [...queryKeys.manga.all, 'latest'] as const,
        search: (query: string) => [...queryKeys.manga.all, 'search', query] as const,
        detail: (id: string) => [...queryKeys.manga.all, id] as const,
        chapters: (id: string, lang: string) => [...queryKeys.manga.all, id, 'chapters', lang] as const,
    },
    chapters: {
        all: ['chapters'] as const,
        pages: (id: string) => [...queryKeys.chapters.all, id, 'pages'] as const,
        navigation: (id: string) => [...queryKeys.chapters.all, id, 'navigation'] as const,
    },
    user: {
        all: ['user'] as const,
        me: () => [...queryKeys.user.all, 'me'] as const,
        library: () => [...queryKeys.user.all, 'library'] as const,
        bookmarks: () => [...queryKeys.user.all, 'bookmarks'] as const,
        progress: () => [...queryKeys.user.all, 'progress'] as const,
        history: () => [...queryKeys.user.all, 'history'] as const,
    },
} as const;

