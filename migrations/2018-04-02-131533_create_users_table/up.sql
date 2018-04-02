
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    banned TIMESTAMPTZ,
    banreason VARCHAR,
    filters TEXT[] NOT NULL DEFAULT '{}',
    groups TEXT[] NOT NULL DEFAULT '{}',
    avatar TEXT,
    description TEXT
);

SELECT diesel_manage_updated_at('users');
