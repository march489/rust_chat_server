// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        room_id -> Integer,
        body -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    room_members (room_id, user_id) {
        room_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    rooms (id) {
        id -> Nullable<Integer>,
        room_name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(posts -> rooms (room_id));
diesel::joinable!(posts -> users (user_id));
diesel::joinable!(room_members -> rooms (room_id));
diesel::joinable!(room_members -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    room_members,
    rooms,
    users,
);
