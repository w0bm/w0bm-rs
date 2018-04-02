
CREATE TABLE tags (
    normalized VARCHAR(30) NOT NULL PRIMARY KEY,
    tag VARCHAR(30) NOT NULL
);

INSERT INTO tags VALUES
    ('nsfw', 'nsfw'),
    ('sfw', 'sfw');