-- Your SQL goes here
CREATE TABLE "posts" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    author TEXT NOT NULL,
    thread TEXT NOT NULL,
    body TEXT NOT NULL--,
    --"created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
