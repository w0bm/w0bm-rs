use chrono::prelude::*;
use db::DbConn;
use diesel::dsl::not;
use diesel::prelude::*;
use schema::*;

#[derive(Debug, Clone, PartialEq, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
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

    pub fn random(filters: &[String], conn: DbConn) -> QueryResult<Self> {
        use schema::videos::dsl::*;

        let c = videos
            .filter(not(tags.overlaps_with(filters)))
            .count()
            .get_result(&*conn)?;
        if c < 1 {
            return Err(::diesel::NotFound);
        }
        let s = ::util::rand_range(0, c);

        videos
            .filter(tags.contains(filters))
            .offset(s)
            .first(&*conn)
    }
}
