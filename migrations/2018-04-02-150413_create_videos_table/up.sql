
CREATE TABLE videos (
    id BIGSERIAL PRIMARY KEY,
    file TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    hash VARCHAR(60) NOT NULL UNIQUE,
    tags TEXT[] NOT NULL DEFAULT '{"sfw"}',
    title VARCHAR(255),
    description TEXT
);
CREATE INDEX ON videos USING GIN ("tags");

SELECT diesel_manage_updated_at('videos');
