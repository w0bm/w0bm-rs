use chrono::{DateTime, Utc};
use diesel::prelude::*;
use jwt::{decode, Validation};
use rocket::request::{self, FromParam, FromRequest, Request};
use rocket::{Outcome, State, http::{RawStr, Status}};
use db::DbConn;
use schema::*;
use util::*;

#[derive(Debug, Clone, Serialize, Queryable, Identifiable, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip)]
    password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub banned: Option<DateTime<Utc>>,
    pub banreason: Option<String>,
    pub filters: Vec<String>,
    pub groups: Vec<String>,
    pub avatar: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

type WithId = ::diesel::dsl::Eq<users::id, i64>;
type ById = ::diesel::dsl::Filter<users::table, WithId>;
type WithUsername<'a> = ::diesel::dsl::Eq<users::username, &'a str>;
type ByUsername<'a> = ::diesel::dsl::Filter<users::table, WithUsername<'a>>;

impl User {
    pub fn with_id(id: i64) -> WithId {
        users::id.eq(id)
    }
    pub fn by_id(id: i64) -> ById {
        ::schema::users::dsl::users.filter(Self::with_id(id))
    }
    pub fn with_username(username: &str) -> WithUsername {
        users::username.eq(username)
    }
    pub fn by_username(username: &str) -> ByUsername {
        ::schema::users::dsl::users.filter(Self::with_username(username))
    }
    pub fn check_pw(&self, pw: &[u8]) -> bool {
        verify_password(&self.password, pw)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Hash)]
pub struct Token {
    pub user_id: i64,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<User, ()> {
        use schema::users::dsl::*;
        let token = match req.headers().get_one("Authorization") {
            Some(a) if a.starts_with("Bearer ") => &a[7..],
            _ => return Outcome::Forward(()),
        };
        let key = req.guard::<State<Secret>>()?;
        let token = match decode::<Token>(token, &**key, &Validation::default()) {
            Ok(t) => t.claims.user_id,
            _ => return Outcome::Forward(()),
        };
        let conn = req.guard::<DbConn>()?;

        match users.filter(id.eq(token)).first(&*conn) {
            Ok(u) => Outcome::Success(u),
            Err(_) => Outcome::Failure((Status::NotFound, ())),
        }
    }
}

impl<'a> FromParam<'a> for User {
    type Error = ();
    fn from_param_with_request(param: &'a RawStr, req: &'a Request) -> Result<Self, Self::Error> {
        use schema::users::dsl::*;
        let uname = param.url_decode().map_err(|_| ())?;
        let conn = match req.guard::<DbConn>() {
            Outcome::Success(c) => c,
            _ => return Err(()),
        };

        users
            .filter(username.eq(uname))
            .first(&*conn)
            .map_err(|_| ())
    }
    fn from_param(_: &'a RawStr) -> Result<Self, Self::Error> {
        unreachable!()
    }
}
