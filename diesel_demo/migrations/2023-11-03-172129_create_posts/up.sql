-- Your SQL goes here
CREATE TABLE "posts" (
	"id"	INTEGER NOT NULL,
	"title"	TEXT NOT NULL,
	"body"	TEXT NOT NULL,
	"published"	INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "users" (
    "id"        INTEGER NOT NULL,
    "name"      TEXT NOT NULL,
    "password"  TEXT NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT)
);
