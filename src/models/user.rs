use chrono::{Utc, DateTime};
use rocket::{State, Outcome, http::{RawStr, Status}};
use rocket::request::{self, FromRequest, Request, FromParam};
use diesel::prelude::*;
use ::schema::users;
use ::db::DbConn;
use ::util::*;
use jwt::{Validation, decode};

#[derive(Debug, Clone, Serialize, Queryable, Identifiable, PartialEq)]
pub struct User {
    id: i64,
    username: String,
    #[serde(skip)]
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
    banned: Option<DateTime<Utc>>,
    banreason: Option<String>,
    filters: Vec<String>,
    groups: Vec<String>,
    avatar: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    username: &'a str,
    password: &'a str,
}

impl User {
    pub fn check_pw(&self, pw: &[u8]) -> bool {
        verify_password(&self.password, pw)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Hash)]
struct Token {
    user_id: i64
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<User, ()> {
        use ::schema::users::dsl::*;
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
        use ::schema::users::dsl::*;
        let uname = param.url_decode().map_err(|_| ())?;
        let conn = match req.guard::<DbConn>() {
            Outcome::Success(c) => c,
            _ => return Err(()),
        };

        users.filter(username.eq(uname))
            .first(&*conn)
            //.map(|u| User { password: "".into(), ..u })
            .map_err(|_| ())
    }
    fn from_param(_: &'a RawStr) -> Result<Self, Self::Error> {
        unreachable!()
    }
}
