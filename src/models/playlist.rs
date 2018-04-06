use super::user::User;
use super::video::Video;
use chrono::{DateTime, Utc};
use db::DbConn;
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
    pub fn random_video(&self, filter: &[String], conn: DbConn) -> QueryResult<PlaylistMessage> {
        use diesel::dsl::not;
        use schema::playlist_video as pv;
        use schema::videos::dsl as v;

        let query = v::videos.inner_join(pv::table).filter(
            pv::playlist_id
                .eq(self.id)
                .and(not(v::tags.overlaps_with(filter))),
        );

        let c = query.count().get_result(&*conn)?;

        if c < 1 {
            return Err(::diesel::NotFound);
        }
        let s = rand_range(0, c);

        let prev_exists = s != 0;
        let next_exists = s != c - 1;

        let limit = 1 + [prev_exists, next_exists]
            .into_iter()
            .filter(|&&e| e)
            .count() as i64;

        let query2 = query.offset(s).limit(limit);
        let mut videos: Vec<(Video, PlaylistVideo)> = if self.editable {
            query2
                .order((pv::ordering.asc(), pv::created_at.asc()))
                .load(&*conn)?
        } else {
            query2.order(pv::created_at.asc()).load(&*conn)?
        };

        let prev = if prev_exists {
            Some(videos[0].0.id)
        } else {
            None
        };

        let next = if next_exists {
            Some(videos[2].0.id)
        } else {
            None
        };

        let video = if prev_exists {
            videos.remove(1).0
        } else {
            videos.remove(0).0
        };

        let first = if !prev_exists {
            None
        } else if s == 0 {
            prev
        } else if self.editable {
            let (v, _): (Video, PlaylistVideo) = query
                .order((pv::ordering.asc(), pv::created_at.asc()))
                .first(&*conn)?;
            Some(v.id)
        } else {
            let (v, _): (Video, PlaylistVideo) = query.order(pv::created_at.asc()).first(&*conn)?;
            Some(v.id)
        };

        let last = if !next_exists {
            None
        } else if s == c - 1 {
            next
        } else if self.editable {
            let (v, _): (Video, PlaylistVideo) = query
                .order((pv::ordering.desc(), pv::created_at.desc()))
                .first(&*conn)?;
            Some(v.id)
        } else {
            let (v, _): (Video, PlaylistVideo) = query.order(pv::created_at.desc()).first(&*conn)?;
            Some(v.id)
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
