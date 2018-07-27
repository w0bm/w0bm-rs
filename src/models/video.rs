use chrono::prelude::*;
use diesel::dsl::not;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::*;

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

type WithId = ::diesel::dsl::Eq<videos::id, i64>;
type ById = ::diesel::dsl::Filter<videos::table, WithId>;

impl Video {
    pub fn with_id(id: i64) -> WithId {
        videos::id.eq(id)
    }

    pub fn by_id(id: i64) -> ById {
        videos::dsl::videos.filter(Self::with_id(id))
    }

    pub fn random(filters: &[String], conn: &PgConnection) -> QueryResult<PlaylistMessage> {
        use schema::videos::dsl::*;

        let query = videos.filter(not(tags.overlaps_with(filters)));

        let c = query.count().get_result(conn)?;
        if c < 1 {
            return Err(::diesel::NotFound);
        }

        // Find best option to do this (probably limit 3 always, and clamp s between [0, c))
        // then just count result
        let (limit, s) = match ::util::rand_range(0, c) {
            0 => (2, 0),
            n => (3, n - 1),
        };

        let mut vids: Vec<Self> = query
            .order(created_at.asc())
            .offset(s)
            .limit(limit)
            .load(conn)?;

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
            _ => Some(query.select(id).order(created_at.asc()).first(conn)?),
        };

        let last = match next {
            None => None,
            n if s == c - 2 => n,
            _ => Some(query.select(id).order(created_at.desc()).first(conn)?),
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

impl<'a> FromParam<'a> for Video {
    type Error = ();
    fn from_param_with_request(param: &'a RawStr, req: &'a Request) -> Result<Self, Self::Error> {
        let vid = param.parse().map_err(|_| ())?;
        let conn = match req.guard::<DbConn>() {
            Outcome::Success(c) => c,
            _ => return Err(()),
        };

        Self::by_id(vid).first(&*conn).map_err(|_| ())
    }

    fn from_param(_: &'a RawStr) -> Result<Self, Self::Error> {
        unreachable!()
    }
}
