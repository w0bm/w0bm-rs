
CREATE TABLE messages (
    id BIGSERIAL PRIMARY KEY,
    from_user BIGINT REFERENCES users (id),
    to_user BIGINT NOT NULL REFERENCES users (id),
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(), -- read if created_at != updated_at
    deleted_at TIMESTAMPTZ
);
