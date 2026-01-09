import { useInfiniteQuery, useQuery } from '@tanstack/react-query';
import { mangaApi } from '@/lib/api';
import { queryKeys } from '@/lib/api/query-keys';

export const usePopularManga = () => {
    return useInfiniteQuery({
        queryKey: queryKeys.manga.popular(),
        queryFn: ({ pageParam = 0 }) => mangaApi.getPopular(20, pageParam),
        getNextPageParam: (lastPage) => {
            const nextOffset = lastPage.offset + lastPage.limit;
            return nextOffset < lastPage.total ? nextOffset : undefined;
        },
        initialPageParam: 0,
    });
};

export const useLatestManga = () => {
    return useInfiniteQuery({
        queryKey: queryKeys.manga.latest(),
        queryFn: ({ pageParam = 0 }) => mangaApi.getLatest(20, pageParam),
        getNextPageParam: (lastPage) => {
            const nextOffset = lastPage.offset + lastPage.limit;
            return nextOffset < lastPage.total ? nextOffset : undefined;
        },
        initialPageParam: 0,
    });
};

export const useMangaSearch = (query: string, enabled = true) => {
    return useInfiniteQuery({
        queryKey: queryKeys.manga.search(query),
        queryFn: ({ pageParam = 0 }) => mangaApi.search(query, 20, pageParam),
        getNextPageParam: (lastPage) => {
            const nextOffset = lastPage.offset + lastPage.limit;
            return nextOffset < lastPage.total ? nextOffset : undefined;
        },
        enabled: enabled && query.length > 0,
        initialPageParam: 0,
    });
};

export const useMangaDetails = (id: string) => {
    return useQuery({
        queryKey: queryKeys.manga.detail(id),
        queryFn: () => mangaApi.getDetails(id),
        enabled: !!id,
    });
};

export const useMangaChapters = (id: string, lang: string | null = null) => {
    return useInfiniteQuery({
        queryKey: queryKeys.manga.chapters(id, lang || 'all'),
        queryFn: ({ pageParam = 0 }) => mangaApi.getChapters(id, lang, 100, pageParam),
        getNextPageParam: (lastPage) => {
            const nextOffset = lastPage.offset + lastPage.limit;
            return nextOffset < lastPage.total ? nextOffset : undefined;
        },
        enabled: !!id,
        initialPageParam: 0,
    });
};

export const useMangaAggregate = (id: string, lang: string | null = null) => {
    return useQuery({
        queryKey: queryKeys.manga.aggregate(id, lang || 'all'),
        queryFn: () => mangaApi.getAggregate(id, lang),
        enabled: !!id,
    });
};

export const useChapterPages = (chapterId: string) => {
    return useQuery({
        queryKey: queryKeys.manga.chapterPages(chapterId),
        queryFn: () => mangaApi.getChapterPages(chapterId),
        enabled: !!chapterId,
    });
};

