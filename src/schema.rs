table! {
    comments (id) {
        id -> Int8,
        user_id -> Int8,
        video_id -> Int8,
        response_to -> Nullable<Int8>,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

table! {
    messages (id) {
        id -> Int8,
        from_user -> Nullable<Int8>,
        to_user -> Int8,
        response_to -> Nullable<Int8>,
        title -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

table! {
    playlists (id) {
        id -> Int8,
        title -> Varchar,
        user_id -> Int8,
        editable -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        tags -> Array<Text>,
    }
}

table! {
    playlist_video (playlist_id, video_id) {
        playlist_id -> Int8,
        video_id -> Int8,
        created_at -> Timestamptz,
        ordering -> Nullable<Int8>,
    }
}

table! {
    tags (normalized) {
        normalized -> Varchar,
        tag -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        banned -> Nullable<Timestamptz>,
        banreason -> Nullable<Varchar>,
        filters -> Array<Text>,
        groups -> Array<Text>,
        avatar -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

table! {
    videos (id) {
        id -> Int8,
        file -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        hash -> Varchar,
        tags -> Array<Text>,
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

joinable!(comments -> users (user_id));
joinable!(comments -> videos (video_id));
joinable!(playlist_video -> playlists (playlist_id));
joinable!(playlist_video -> videos (video_id));
joinable!(playlists -> users (user_id));


allow_tables_to_appear_in_same_query!(
    comments,
    messages,
    playlists,
    playlist_video,
    tags,
    users,
    videos,
);
