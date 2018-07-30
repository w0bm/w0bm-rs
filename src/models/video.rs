use chrono::prelude::*;
use db::GetResult;
use diesel::prelude::*;
use failure::Error;
use futures::future::{result, Future};
use schema::*;


#[derive(Debug, Clone, PartialEq, Serialize, Queryable, Identifiable, Deserialize)]
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

    pub fn random(
        filters: Vec<String>,
        state: &::AppState,
    ) -> impl Future<Item = ::serde_json::Value, Error = Error> {
        state
            .db
            .send(GetResult::new(::diesel::select(filter_random(filters))))
            .from_err()
            .and_then(result)
    }
}
