// @generated automatically by Diesel CLI.

diesel::table! {
    jobs (types) {
        types -> Int4,
    }
}

diesel::table! {
    videos (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        removed -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    jobs,
    videos,
);
