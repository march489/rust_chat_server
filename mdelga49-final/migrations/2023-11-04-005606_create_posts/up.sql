-- Your SQL goes here
CREATE TABLE "posts" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "author" TEXT NOT NULL,
    "thread" TEXT NOT NULL,
    "body" TEXT NOT NULL,
    "timestamp" INTEGER NOT NULL
);
