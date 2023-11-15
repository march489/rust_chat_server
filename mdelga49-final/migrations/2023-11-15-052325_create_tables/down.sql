-- This file should undo anything in `up.sql`
DROP TABLE posts;
DROP INDEX idx_rooms_name;
DROP TABLE rooms;
DROP INDEX idx_users_email;
DROP TABLE users;