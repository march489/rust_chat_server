// @generated automatically by Diesel CLI.
// pub mod schema;

diesel::table! {
    posts (id) {
        id -> Integer,
        author -> Text,
        thread -> Text,
        body -> Text,
        timestamp -> Integer,
    }
}
