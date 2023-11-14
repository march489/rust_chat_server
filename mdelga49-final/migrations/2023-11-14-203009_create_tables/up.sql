-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL,
    password TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_users_email ON users(email);

CREATE TABLE rooms (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_name TEXT NOT NULL
);

CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    room_id INTEGER NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT posts_user_id_fk FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT posts_room_id_fk FOREIGN KEY (room_id) REFERENCES rooms(id)
);


CREATE TABLE room_members (
    room_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    CONSTRAINT rooms_members_pk PRIMARY KEY (room_id, user_id),
    CONSTRAINT room_members_room_fk FOREIGN KEY (room_id) REFERENCES rooms(id),
    CONSTRAINT room_members_user_fk FOREIGN KEY (user_id) REFERENCES users(id)
);