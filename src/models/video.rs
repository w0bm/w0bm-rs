use chrono::prelude::*;
use diesel::dsl::not;
use diesel::prelude::*;
use schema::*;
use db::DbExecutor;
use futures::future::{result, Future};
use failure::Error;
use actix_web::actix::*;

use super::playlist::PlaylistMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Queryable, Identifiable)]
pub struct Video {
    pub id: i64,
    pub file: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub hash: String,
    pub tags: Vec<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

struct RandomVideo(Vec<String>);

impl Message for RandomVideo {
    type Result = Result<PlaylistMessage, Error>;
}

impl Handler<RandomVideo> for DbExecutor {
    type Result = <RandomVideo as Message>::Result;

    fn handle(&mut self, RandomVideo(filters): RandomVideo, _: &mut Self::Context) -> Self::Result {
        use schema::videos::dsl::*;
        let conn = self.0.get()?;
        let query = videos.filter(not(tags.overlaps_with(&filters)));

        let c = query.count().get_result(&conn)?;
        if c < 1 {
            return Err(::diesel::NotFound.into());
        }

        // Find best option to do this (probably limit 3 always, and clamp s between [0, c))
        // then just count result
        let (limit, s) = match ::util::rand_range(0, c) {
            0 => (2, 0),
            n => (3, n - 1),
        };

        let mut vids: Vec<Video> = query
            .order(created_at.asc())
            .offset(s)
            .limit(limit)
            .load(&conn)?;

        let prev = if limit == 3 { Some(vids[0].id) } else { None };
        let next = if limit == 2 && vids.len() == 2 {
            Some(vids[1].id)
        } else if limit == 3 && vids.len() == 3 {
            Some(vids[2].id)
        } else {
            None
        };

        let first = match prev {
            None => None,
            p if s == 0 => p,
            _ => Some(query.select(id).order(created_at.asc()).first(&conn)?),
        };

        let last = match next {
            None => None,
            n if s == c - 2 => n,
            _ => Some(query.select(id).order(created_at.desc()).first(&conn)?),
        };

        let video = if limit == 2 {
            vids.remove(0)
        } else {
            vids.remove(1)
        };

        Ok(PlaylistMessage {
            video,
            prev,
            first,
            next,
            last,
        })
    }
}

type WithId = ::diesel::dsl::Eq<videos::id, i64>;
type ById = ::diesel::dsl::Filter<videos::table, WithId>;

impl Video {
    pub fn with_id(id: i64) -> WithId {
        videos::id.eq(id)
    }

    pub fn by_id(id: i64) -> ById {
        videos::dsl::videos.filter(Self::with_id(id))
    }

    pub fn random(filters: Vec<String>, state: &::AppState) -> impl Future<Item=PlaylistMessage, Error=Error> {
        state.db.send(RandomVideo(filters))
            .from_err()
            .and_then(result)
    }
}

