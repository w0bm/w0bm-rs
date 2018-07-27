use actix_web::{HttpResponse, Json, Responder, State};
use failure::Error;
use futures::future::{err, ok, result};
use futures::Future;
use jwt::{encode, Header};
use models::user::{NewUser, Token, User};
use util::{hash_password};

pub fn login(
    (state, creds): (State<::AppState>, Json<NewUser>),
) -> Box<Future<Item = impl Responder, Error = Error>> {
    let NewUser { username, password } = creds.into_inner();
    Box::new(
        state
            .db
            .send(::db::First::new(User::by_username(username)))
            .from_err()
            .and_then(result)
            .then(move |r: Result<User, _>| match r {
                Ok(u) =>
                    if u.check_pw(password.as_bytes()) {
                        ok(u)
                    } else {
                        err(format_err!("Invalid Password"))
                    },
                Err(e) => err(e),
            })
            .map(move |u| {
                let token = encode(&Header::default(), &Token { user_id: u.id }, &*state.key);
                match token {
                    Ok(t) => HttpResponse::Ok().json(t),
                    Err(e) => HttpResponse::Conflict()
                        .json(json!({ "error": format!("JWT Error: {}", e) })),
                }
            }),
    )
}

pub fn register(
    (state, creds): (State<::AppState>, Json<NewUser>),
) -> Box<Future<Item = impl Responder, Error = Error>> {
    use schema::users::dsl::*;
    let mut creds = creds.into_inner();
    creds.password = match hash_password(creds.password.as_bytes()) {
        Ok(p) => p,
        Err(e) => return Box::new(err(e.into())),
    };
    Box::new(
        state
            .db
            .send(::db::Execute(::diesel::insert_into(users).values(creds)))
            .from_err()
            .map(|_| {
                HttpResponse::Ok().json(json!({
                    "success": "Registration successful"
                }))
            }),
    )
}
