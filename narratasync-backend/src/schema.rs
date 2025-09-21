// @generated automatically by Diesel CLI.

diesel::table! {
    scenario (id) {
        id -> Nullable<Text>,
        title -> Text,
        thumbnail -> Nullable<Text>,
        author -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Text>,
        username -> Text,
        display_name -> Nullable<Text>,
        email -> Nullable<Text>,
        token -> Text,
    }
}

diesel::joinable!(scenario -> users (author));

diesel::allow_tables_to_appear_in_same_query!(scenario, users,);
