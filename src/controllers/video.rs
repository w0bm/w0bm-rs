use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::Json;

use db::DbConn;
use models::playlist::PlaylistMessage;
use models::tag::Tag;
use models::user::User;
use models::video::Video;
use std::error::Error;

#[get("/video/random")]
pub fn video_random(
    conn: DbConn,
    u: Option<User>,
) -> Result<Json<PlaylistMessage>, status::Custom<String>> {
    let def_filter = vec!["nsfw".into()];
    let f = u.map(|usr| usr.filters).unwrap_or(def_filter);

    Video::random(&f, &conn)
        .map(Json)
        .map_err(|e| status::Custom(Status::NotFound, format!("{}", e.description())))
}

#[get("/video/<video>")]
pub fn video_id(video: Video) -> Json<Video> {
    Json(video)
}

#[get("/video/<video>/tags")]
pub fn video_tags(video: Video, conn: DbConn) -> Json<Vec<Tag>> {
    use diesel::dsl::any;
    use diesel::prelude::*;
    use schema::tags::dsl::*;

    // TODO: Error handling. get rid of expect
    Json(
        tags.filter(normalized.eq(any(&video.tags)))
            .load(&*conn)
            .expect("Could not get tags :("),
    )
}
