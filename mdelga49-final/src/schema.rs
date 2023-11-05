// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Nullable<Integer>,
        author -> Text,
        thread -> Text,
        body -> Text,
    }
}
