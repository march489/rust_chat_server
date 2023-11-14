-- This file should undo anything in `up.sql`
DROP TABLE room_members;
DROP TABLE posts;
DROP TABLE rooms;
DROP INDEX idx_users_email;
DROP TABLE users;