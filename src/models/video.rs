
use chrono::prelude::*;
use diesel::prelude::*;
use schema::*;

#[derive(Debug, Clone, PartialEq, Serialize, Queryable, Identifiable, Associations)]
pub struct Video {
    id: i64,
    file: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
    hash: String,
    tags: Vec<String>,
    title: Option<String>,
    description: Option<String>,
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

//     pub fn all() -> All {
//         videos::table
//     }
}
