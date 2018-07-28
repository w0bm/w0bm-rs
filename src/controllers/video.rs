
use diesel::prelude::*;
use models::comment::Comment;
use models::user::User;
use models::video::Video;
use models::tag::Tag;
use futures::future::{result, Future};
use actix_web::{State, Responder, HttpResponse, Path};
use db::{Load, First};
use failure::Error;

pub fn video_random((state, u): (State<::AppState>, Option<User>))
    -> impl Future<Item=impl Responder, Error=Error>
{
    let def_filter = vec!["nsfw".into()];
    let f = u.map(|usr| usr.filters).unwrap_or(def_filter);

    Video::random(f, &state)
        .map(|p| HttpResponse::Ok().json(p))
        // .map_err(|e| HttpResponse::Conflict().json(json!({
        //     "error": format!("Error: {}", e)
        // })))
}

pub fn video_id((state, id): (State<::AppState>, Path<i64>))
    -> impl Future<Item=impl Responder, Error=Error>
{
    state.db.send(First::new(Video::by_id(id.into_inner())))
        .from_err()
        .and_then(result)
        .map(|v: Video| HttpResponse::Ok().json(v))
        // .map_err(|e| HttpResponse::NotFound().json(json!({
        //     "error": format!("Error: {}", e)
        // })))
}

pub fn video_tags((state, id): (State<::AppState>, Path<i64>))
    -> impl Future<Item=impl Responder, Error=Error>
{
    use diesel::dsl::any;
    use schema::tags::dsl::*;

    state.db.send(First::new(Video::by_id(id.into_inner())))
        .from_err()
        .and_then(result)
        .and_then(move |v: Video|
            state.db.send(Load::new(tags.filter(normalized.eq(any(v.tags)))))
                .from_err()
                .and_then(result)
        )
        .map(|t: Vec<Tag>| HttpResponse::Ok().json(json!({
            "tags": t
        })))
        // .map_err(|e| HttpResponse::NotFound().json(json!({
        //     "error": format!("Error: {}", e)
        // })))

}

pub fn video_comments(
    (state, id): (State<::AppState>, Path<i64>)
) -> impl Future<Item=impl Responder, Error=Error>
{
    state.db.send(Load::new(Comment::of_video(id.into_inner())))
        .from_err()
        .and_then(result)
        .map(|c: Vec<Comment>| HttpResponse::Ok().json(json!({
            "comments": c
        })))
        // .map_err(|e| HttpResponse::NotFound().json(json!({
        //     "error": format!("Error: {}", e)
        // })))
}
