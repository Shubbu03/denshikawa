export const APP_CONFIG = {
    DEFAULT_PAGE_SIZE: 20,
    MAX_PAGE_SIZE: 100,
    DEFAULT_LANGUAGE: 'en',
} as const;

export const READER_SETTINGS = {
    FIT_MODES: ['width', 'height', 'contain', 'cover'] as const,
    DEFAULT_FIT_MODE: 'width' as const,
    DEFAULT_DARK_MODE: false,
} as const;

export const MANGA_STATUS = {
    ONGOING: 'ongoing',
    COMPLETED: 'completed',
    CANCELLED: 'cancelled',
    HIATUS: 'hiatus',
} as const;

