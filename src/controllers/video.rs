
use diesel::prelude::*;
use models::comment::Comment;
use models::playlist::PlaylistMessage;
use models::tag::Tag;
use models::user::User;
use models::video::Video;
use std::error::Error;

pub fn video_random(
    u: Option<User>,
) -> Result<Json<PlaylistMessage>, status::Custom<String>> {
    let def_filter = vec!["nsfw".into()];
    let f = u.map(|usr| usr.filters).unwrap_or(def_filter);

    Video::random(&f, &conn)
        .map(Json)
        .map_err(|e| status::Custom(Status::NotFound, format!("{}", e.description())))
}

pub fn video_id(video: Video) -> Json<Video> {
    Json(video)
}

pub fn video_tags(video: Video) -> Result<Json<Vec<Tag>>, status::Custom<String>> {
    use diesel::dsl::any;
    use schema::tags::dsl::*;

    // TODO: Change to InternalServerError and Empty Vec instead of 404
    tags.filter(normalized.eq(any(&video.tags)))
        .load(&*conn)
        .map(Json)
        .map_err(|e| status::Custom(Status::NotFound, format!("{}", e.description())))
}

pub fn video_comments(
    video: Video,
) -> Result<Json<Vec<Comment>>, status::Custom<String>> {
    // TODO: Change to InternalServerError and Empty Vec instead of 404
    Comment::of_video(&video)
        .load(&*conn)
        .map(Json)
        .map_err(|e| status::Custom(Status::NotFound, format!("{}", e.description())))
}
