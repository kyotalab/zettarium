-- Your SQL goes here
CREATE TABLE zettel_tags (
    zettel_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (zettel_id, tag_id),
    FOREIGN KEY(zettel_id) REFERENCES zettels(id),
    FOREIGN KEY(tag_id) REFERENCES tags(id)
);