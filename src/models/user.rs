use actix_web::{http::header, Error as ActixError, FromRequest, HttpRequest};
use chrono::{DateTime, Utc};
use db;
use diesel::prelude::*;
use futures::future::{err, ok, result, Future};
use jwt::{decode, Validation};
use schema::users;
use std::ops::Deref;
use util::*;
use AppState;

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

pub struct Admin(pub User);

impl FromRequest<AppState> for Admin {
    type Config = ();
    type Result = Box<Future<Item = Admin, Error = ActixError>>;
    fn from_request(req: &HttpRequest<AppState>, _: &Self::Config) -> Self::Result {
        Box::new(User::extract(req).then(|u| match u {
            Ok(user) => if user.groups.iter().any(|g| g == "admin") {
                ok(Admin(user))
            } else {
                err(format_err!("User is not an Admin").into())
            },
            Err(e) => err(e),
        }))
    }
}

impl Deref for Admin {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

type WithId = ::diesel::dsl::Eq<users::id, i64>;
type ById = ::diesel::dsl::Filter<users::table, WithId>;
type WithUsername = ::diesel::dsl::Eq<users::username, String>;
type ByUsername = ::diesel::dsl::Filter<users::table, WithUsername>;

impl User {
    pub fn with_id(id: i64) -> WithId {
        users::id.eq(id)
    }
    pub fn by_id(id: i64) -> ById {
        users::dsl::users.filter(Self::with_id(id))
    }
    pub fn with_username(username: String) -> WithUsername {
        users::username.eq(username)
    }
    pub fn by_username(username: String) -> ByUsername {
        users::dsl::users.filter(Self::with_username(username))
    }
    pub fn check_pw(&self, pw: &[u8]) -> bool {
        verify_password(&self.password, pw)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Hash)]
pub struct Token {
    pub user_id: i64,
}

impl FromRequest<AppState> for User {
    type Config = ();
    type Result = Box<Future<Item = User, Error = ActixError>>;

    fn from_request(req: &HttpRequest<AppState>, _: &Self::Config) -> Self::Result {
        use self::users::dsl;
        let token = match req.headers().get(header::AUTHORIZATION) {
            Some(a) => a.to_str(),
            None => return Box::new(err(format_err!("Missing Header").into())),
        };
        let token = match token {
            Ok(t) if t.starts_with("Bearer ") => &t[7..],
            Ok(t) => return Box::new(err(format_err!("Invalid Header: {}", t).into())),
            Err(e) => return Box::new(err(format_err!("Error: {}", e).into())),
        };
        let ref key = req.state().key;
        let token = match decode::<Token>(token, &**key, &Validation::default()) {
            Ok(t) => t.claims.user_id,
            Err(e) => return Box::new(err(format_err!("Error: {}", e).into())),
        };

        let q = db::First::new(dsl::users.filter(dsl::id.eq(token)));
        Box::new(
            req.state()
                .db
                .send(q)
                .from_err()
                .and_then(|res| result(res.map_err(From::from))),
        )
    }
}

// impl<'a> FromParam<'a> for User {
//     type Error = ();
//     fn from_param_with_request(param: &'a RawStr, req: &'a Request) -> Result<Self, Self::Error> {
//         use self::users::dsl;
//         let uname = param.url_decode().map_err(|_| ())?;
//         let conn = match req.guard::<DbConn>() {
//             Outcome::Success(c) => c,
//             _ => return Err(()),
//         };

//         dsl::users
//             .filter(dsl::username.eq(uname))
//             .first(&*conn)
//             .map_err(|_| ())
//     }
//     fn from_param(_: &'a RawStr) -> Result<Self, Self::Error> {
//         unreachable!()
//     }
// }
