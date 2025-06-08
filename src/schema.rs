// @generated automatically by Diesel CLI.

diesel::table! {
    tags (id) {
        id -> Text,
        tag_name -> Text,
    }
}

diesel::table! {
    zettel_tags (zettel_id, tag_id) {
        zettel_id -> Text,
        tag_id -> Text,
    }
}

diesel::table! {
    zettels (id) {
        id -> Text,
        title -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        archived -> Bool,
    }
}

diesel::joinable!(zettel_tags -> tags (tag_id));
diesel::joinable!(zettel_tags -> zettels (zettel_id));

diesel::allow_tables_to_appear_in_same_query!(
    tags,
    zettel_tags,
    zettels,
);
