extern crate actix_web;
extern crate chrono;
extern crate dotenv;

extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate argon2;

#[macro_use]
extern crate failure;
extern crate futures;
extern crate num_cpus;

#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate diesel;
extern crate r2d2;

extern crate jsonwebtoken as jwt;
extern crate rand;
extern crate ring;
extern crate slug;

use actix_web::actix::*;
use actix_web::{middleware, server, App};

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

mod controllers;
mod db;
mod models;
mod schema;
mod util;

pub struct AppState {
    pub db: Addr<db::DbExecutor>,
    pub key: util::Secret,
}

fn main() {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    dotenv::dotenv().ok();

    let key = util::generate_secret().expect("Error generating random secret");

    let manager = ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL").expect("DATABASE_URL is missing"));
    let pool = db::Pool::new(manager).expect("Could not connect to Database 2");

    let sys = System::new("w0bm-rs");

    let addr = SyncArbiter::start(num_cpus::get(), move || db::DbExecutor(pool.clone()));

    server::new(move || {
        App::with_state(AppState {
            db: addr.clone(),
            key: key.clone(),
        }).middleware(middleware::Logger::default())
            .scope("/auth", |scop| {
                scop.resource("/login", |r| r.post().with_async(controllers::user::login))
                    .resource("/register", |r| {
                        r.post().with_async(controllers::user::register)
                    })
            })
            .scope("/api/v1", |s| s)
    }).bind("[::]:8000")
        .expect("Could not bind")
        .start();

    sys.run();
    // rocket::ignite()
    //     .mount(
    //         "/api/v1",
    //         routes![
    //             controllers::video::video_random,
    //             controllers::video::video_id,
    //             controllers::video::video_tags,
    //             controllers::video::video_comments,
    //         ],
    //     )
    //     .manage(db::init_pool())
    //     .manage(key)
    //     .launch();
}
