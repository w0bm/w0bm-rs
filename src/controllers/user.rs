use db::DbConn;
use diesel::prelude::*;
use jwt::{encode, Header};
use models::user::{NewUser, Token, User};
use rocket::State;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::Json;
use util::{hash_password, Secret};

#[post("/login", format = "application/json", data = "<creds>")]
pub fn login(
    creds: Json<NewUser>,
    conn: DbConn,
    key: State<Secret>,
) -> Result<Json<String>, status::Custom<String>> {
    let u: User = User::by_username(&creds.username)
        .first(&*conn)
        .map_err(|_| status::Custom(Status::Unauthorized, "User not found".into()))?;

    if !u.check_pw(&creds.password.as_bytes()) {
        return Err(status::Custom(
            Status::Unauthorized,
            "Invalid Password".into(),
        ));
    }

    let token = encode(&Header::default(), &Token { user_id: u.id }, &**key)
        .map_err(|e| status::Custom(Status::InternalServerError, format!("{}", e)))?;

    Ok(Json(token))
}

#[post("/register", format = "application/json", data = "<creds>")]
pub fn register(
    creds: Json<NewUser>,
    conn: DbConn,
) -> Result<Json<&'static str>, status::Custom<&'static str>> {
    use schema::users::dsl::*;
    let mut creds = creds.into_inner();
    creds.password = hash_password(creds.password.as_bytes())
        .map_err(|_| status::Custom(Status::InternalServerError, "Could not create user"))?;
    ::diesel::insert_into(users)
        .values(&creds)
        .execute(&*conn)
        .map_err(|_| status::Custom(Status::InternalServerError, "Could not create user"))
        .map(|_| Json("Registration successful"))
}
