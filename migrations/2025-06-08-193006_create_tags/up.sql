-- Your SQL goes here
CREATE TABLE tags (
    id TEXT NOT NULL PRIMARY KEY,
    tag_name TEXT NOT NULL UNIQUE
);