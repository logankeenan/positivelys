table! {
    positivelys (id) {
        id -> Integer,
        moment -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    media_files (id) {
        id -> Integer,
        positively_id -> Integer,
        file_name -> Text,
        file_location -> Text,
        file_extension -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(media_files -> positivelys (positively_id));

allow_tables_to_appear_in_same_query!(
    positivelys,
    media_files,
);
