// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        author -> Text,
        thread -> Text,
        body -> Text,
    }
}
