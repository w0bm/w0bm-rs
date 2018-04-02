
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initializes a database pool
fn init_pool() -> Pool {
    ::dotenv::dotenv().ok();

    let manager = ConnectionManager::<PgConnection>::new(env!("DATABASE_URL", ""));
    r2d2::Pool::new(manager).expect("Could not initialize Database")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// Attemts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
