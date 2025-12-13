export interface ApiError {
    message: string;
    code?: string;
    status?: number;
}

export interface PaginatedResponse<T> {
    data: T[];
    total: number;
    limit: number;
    offset: number;
}

export interface ApiResponse<T> {
    data: T;
    message?: string;
}

