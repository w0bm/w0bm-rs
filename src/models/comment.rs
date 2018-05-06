use super::user::User;
use super::video::Video;
use chrono::prelude::*;
use diesel::prelude::*;
use schema::*;

#[derive(Debug, Clone, PartialEq, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Video)]
pub struct Comment {
    pub id: i64,
    pub user_id: i64,
    pub video_id: i64,
    pub response_to: Option<i64>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

type WithVideoId<'a> = ::diesel::dsl::Eq<comments::video_id, &'a i64>;
type ByVideoId<'a> = ::diesel::dsl::Filter<comments::table, WithVideoId<'a>>;
type WithUserId<'a> = ::diesel::dsl::Eq<comments::user_id, &'a i64>;
type ByUserId<'a> = ::diesel::dsl::Filter<comments::table, WithUserId<'a>>;

impl Comment {
    pub fn of_video(v: &Video) -> ByVideoId {
        Self::belonging_to(v)
    }
    pub fn of_user(u: &User) -> ByUserId {
        Self::belonging_to(u)
    }
}
