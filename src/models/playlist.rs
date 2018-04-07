use super::user::User;
use super::video::Video;
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::{playlist_video, playlists};
use util::rand_range;

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize, Associations)]
#[belongs_to(User)]
pub struct Playlist {
    pub id: i64,
    pub title: String,
    pub user_id: i64,
    pub editable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize, Associations)]
#[table_name = "playlist_video"]
#[primary_key(playlist_id, video_id)]
#[belongs_to(Playlist)]
#[belongs_to(Video)]
pub struct PlaylistVideo {
    pub playlist_id: i64,
    pub video_id: i64,
    pub created_at: DateTime<Utc>,
    pub ordering: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PlaylistMessage {
    pub first: Option<i64>,
    pub prev: Option<i64>,
    pub next: Option<i64>,
    pub last: Option<i64>,
    pub video: Video,
}

impl PlaylistMessage {
    pub fn new(video: Video) -> Self {
        Self {
            first: None,
            prev: None,
            next: None,
            last: None,
            video,
        }
    }
}

type WithId = ::diesel::dsl::Eq<playlists::id, i64>;
type ById = ::diesel::dsl::Filter<playlists::table, WithId>;

impl Playlist {
    pub fn with_id(id: i64) -> WithId {
        playlists::id.eq(id)
    }
    pub fn by_id(id: i64) -> ById {
        playlists::dsl::playlists.filter(Self::with_id(id))
    }
    pub fn random_video(
        &self,
        filter: &[String],
        conn: &PgConnection,
    ) -> QueryResult<PlaylistMessage> {
        use diesel::dsl::not;
        use schema::playlist_video as pv;
        use schema::videos::dsl as v;

        let query = v::videos.inner_join(pv::table).filter(
            pv::playlist_id
                .eq(self.id)
                .and(not(v::tags.overlaps_with(filter))),
        );

        let c = query.count().get_result(conn)?;

        if c < 1 {
            return Err(::diesel::NotFound);
        }

        let (limit, s) = match rand_range(0, c) {
            0 => (2, 0),
            n => (3, n - 1),
        };

        let query2 = query.offset(s).limit(limit);
        let mut vids: Vec<(Video, PlaylistVideo)> = if self.editable {
            query2
                .order((pv::ordering.asc(), pv::created_at.asc()))
                .load(&*conn)?
        } else {
            query2.order(pv::created_at.asc()).load(conn)?
        };

        let prev = if limit == 3 { Some(vids[0].0.id) } else { None };
        let next = if limit == 2 && vids.len() == 2 {
            Some(vids[1].0.id)
        } else if limit == 3 && vids.len() == 3 {
            Some(vids[2].0.id)
        } else {
            None
        };

        let query2 = query2.select(v::id);

        let first = match prev {
            None => None,
            p if s == 0 => p,
            _ => Some(if self.editable {
                query2
                    .order((pv::ordering.asc(), pv::created_at.asc()))
                    .first(conn)?
            } else {
                query2.order(pv::created_at.asc()).first(conn)?
            }),
        };

        let last = match next {
            None => None,
            p if s == c - 2 => p,
            _ => Some(if self.editable {
                query2
                    .order((pv::ordering.desc(), pv::created_at.desc()))
                    .first(conn)?
            } else {
                query2.order(pv::created_at.desc()).first(conn)?
            }),
        };

        let (video, _) = if limit == 2 {
            vids.remove(0)
        } else {
            vids.remove(1)
        };

        Ok(PlaylistMessage {
            video,
            prev,
            next,
            first,
            last,
        })
    }
}
