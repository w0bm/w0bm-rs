use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::Json;

use db::DbConn;
use models::user::User;
use models::video::Video;
use std::error::Error;

#[get("/video/random")]
pub fn video_random(conn: DbConn, u: Option<User>) -> Result<Json<Video>, status::Custom<String>> {
    let def_filter = vec!["nsfw".into()];
    let f = u.map(|usr| usr.filters).unwrap_or(def_filter);

    Video::random(&f, conn)
        .map(|v| Json(v))
        .map_err(|e| status::Custom(Status::NotFound, format!("{}", e.description())))
}

#[get("/video/<video>")]
pub fn video_id(video: Video) -> Json<Video> {
    Json(video)
}
