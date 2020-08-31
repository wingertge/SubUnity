table! {
    changes (id) {
        id -> Nullable<Integer>,
        timestamp -> Timestamp,
        author -> Text,
        changes_json -> Text,
    }
}

table! {
    subtitles (video_id, language) {
        video_id -> Text,
        language -> Text,
        subs_json -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        username -> Text,
        email -> Nullable<Text>,
        picture -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    changes,
    subtitles,
    users,
);
