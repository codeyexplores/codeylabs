-- Add migration script here
CREATE TABLE articles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    link TEXT NOT NULL,
    published TEXT,
    summary TEXT
);