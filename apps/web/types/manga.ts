export interface MangaFilters {
    status?: string;
    tags?: string[];
    year?: number;
    contentRating?: string;
}

export interface MangaSearchParams {
    query: string;
    limit?: number;
    offset?: number;
    filters?: MangaFilters;
}

