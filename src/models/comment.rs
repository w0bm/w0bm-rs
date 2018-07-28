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

type WithVideoId = ::diesel::dsl::Eq<comments::video_id, i64>;
type ByVideoId = ::diesel::dsl::Filter<comments::table, WithVideoId>;
type WithUserId = ::diesel::dsl::Eq<comments::user_id, i64>;
type ByUserId = ::diesel::dsl::Filter<comments::table, WithUserId>;

impl Comment {
    pub fn of_video(vid: i64) -> ByVideoId {
        use schema::comments::dsl::*;
        comments.filter(video_id.eq(vid))
    }
    pub fn of_user(uid: i64) -> ByUserId {
        use schema::comments::dsl::*;
        comments.filter(user_id.eq(uid))
    }
}
