export const queryKeys = {
    manga: {
        all: ['manga'] as const,
        popular: () => [...queryKeys.manga.all, 'popular'] as const,
        latest: () => [...queryKeys.manga.all, 'latest'] as const,
        search: (query: string) => [...queryKeys.manga.all, 'search', query] as const,
        detail: (id: string) => [...queryKeys.manga.all, id] as const,
        chapters: (id: string, lang: string) => [...queryKeys.manga.all, id, 'chapters', lang] as const,
        aggregate: (id: string, lang: string) => [...queryKeys.manga.all, id, 'aggregate', lang] as const,
    },
    user: {
        all: ['user'] as const,
        me: () => [...queryKeys.user.all, 'me'] as const,
    },
} as const;

