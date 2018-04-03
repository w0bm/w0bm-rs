
use chrono::prelude::*;
use diesel::prelude::*;
use schema::*;

#[derive(Debug, Clone, PartialEq, Serialize, Queryable, Identifiable, Associations)]
pub struct Video<'a> {
    id: i64,
    file: &'a str,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
    hash: &'a str,
    tags: &'a [&'a str],
    title: Option<&'a str>,
    description: Option<&'a str>,
}

type AllColumns = (
    videos::id,
    videos::file,
    videos::updated_at,
    videos::created_at,
    videos::deleted_at,
    videos::hash,
    videos::tags,
    videos::title,
    videos::description,
);

const ALL_COLUMNS: AllColumns = (
    videos::id,
    videos::file,
    videos::updated_at,
    videos::created_at,
    videos::deleted_at,
    videos::hash,
    videos::tags,
    videos::title,
    videos::description,
);

type All = ::diesel::dsl::Select<videos::table, AllColumns>;
type WithId = ::diesel::dsl::Eq<videos::id, i64>;
type ById = ::diesel::dsl::Filter<All, WithId>;


impl<'a> Video<'a> {

    pub fn with_id(id: i64) -> WithId {
        videos::id.eq(id)
    }

    pub fn by_id(id: i64) -> ById {
        Self::all().filter(Self::with_id(id))
    }

    pub fn all() -> All {
        videos::table.select(ALL_COLUMNS)
    }
}
