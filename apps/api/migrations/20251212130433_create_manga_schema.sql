-- Create manga status enum
CREATE TYPE manga_status AS ENUM ('ongoing', 'completed', 'hiatus', 'cancelled');

-- Cached manga metadata from MangaDex
CREATE TABLE manga_cache (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    mangadex_id VARCHAR(36) UNIQUE NOT NULL,
    title TEXT NOT NULL,
    alt_titles JSONB,
    description TEXT,
    cover_url TEXT,
    status manga_status,
    year INTEGER,
    content_rating VARCHAR(20),
    tags JSONB,
    author_names JSONB,
    artist_names JSONB,
    cached_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_manga_cache_mangadex_id ON manga_cache(mangadex_id);
CREATE INDEX idx_manga_cache_cached_at ON manga_cache(cached_at);

-- Cached chapter list from MangaDex
CREATE TABLE chapter_cache (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    mangadex_id VARCHAR(36) UNIQUE NOT NULL,
    manga_mangadex_id VARCHAR(36) NOT NULL,
    chapter_number VARCHAR(10),
    volume VARCHAR(10),
    title TEXT,
    language VARCHAR(10) NOT NULL,
    scanlation_group_id VARCHAR(36),
    scanlation_group_name TEXT,
    page_count INTEGER,
    published_at TIMESTAMPTZ,
    cached_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_chapter_manga ON chapter_cache(manga_mangadex_id);
CREATE INDEX idx_chapter_cached_at ON chapter_cache(cached_at);

-- User bookmarks (permanent)
CREATE TABLE user_bookmarks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    manga_mangadex_id VARCHAR(36) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, manga_mangadex_id)
);

CREATE INDEX idx_bookmarks_user ON user_bookmarks(user_id);

-- User reading progress (permanent)
CREATE TABLE user_reading_progress (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    manga_mangadex_id VARCHAR(36) NOT NULL,
    chapter_mangadex_id VARCHAR(36) NOT NULL,
    page_number INTEGER NOT NULL DEFAULT 1,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, manga_mangadex_id)
);

CREATE INDEX idx_progress_user ON user_reading_progress(user_id);

