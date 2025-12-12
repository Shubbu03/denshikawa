-- Reading history: tracks which chapters user has completed reading
CREATE TABLE reading_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    manga_mangadex_id VARCHAR(36) NOT NULL,
    chapter_mangadex_id VARCHAR(36) NOT NULL,
    read_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, chapter_mangadex_id)
);

CREATE INDEX idx_reading_history_user ON reading_history(user_id);
CREATE INDEX idx_reading_history_manga ON reading_history(manga_mangadex_id);
CREATE INDEX idx_reading_history_read_at ON reading_history(read_at DESC);
