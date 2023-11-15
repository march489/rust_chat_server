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
        display_name -> Text,
    }
}

diesel::joinable!(posts -> rooms (room_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(posts, rooms, users,);
