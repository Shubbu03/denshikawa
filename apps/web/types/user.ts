export interface ReadingProgress {
    mangaId: string;
    chapterId: string;
    page: number;
    updatedAt: string;
}

export interface LibraryFilters {
    status?: string;
    sortBy?: 'title' | 'added_at' | 'last_read_at';
    sortOrder?: 'asc' | 'desc';
}

