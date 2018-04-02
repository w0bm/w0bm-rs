
CREATE TABLE playlists (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users (id),
    editable BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    tags TEXT[] NOT NULL DEFAULT '{}'
);
CREATE INDEX ON playlists USING GIN ("tags");

SELECT diesel_manage_updated_at('playlists');

CREATE TABLE playlist_video (
    playlist_id BIGINT NOT NULL REFERENCES playlists (id),
    video_id BIGINT NOT NULL REFERENCES videos (id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    odering BIGINT,
    PRIMARY KEY (playlist_id, video_id)
);

-- Create Uploads and Favorites playlists on user creation
CREATE OR REPLACE FUNCTION create_default_playlists_proc() RETURNS trigger AS $$
BEGIN
    INSERT INTO playlists (title, user_id) VALUES
        ('Uploads', NEW.id),
        ('Favorites', NEW.id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER create_default_playlists AFTER INSERT ON users
    FOR EACH ROW EXECUTE PROCEDURE create_default_playlists_proc();
