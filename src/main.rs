#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate argon2;
extern crate chrono;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate jsonwebtoken as jwt;
extern crate rand;
extern crate ring;

mod controllers;
mod db;
mod models;
mod schema;
mod util;

fn main() {
    let key = util::generate_secret().expect("Error generating random secret");
    rocket::ignite()
        .mount(
            "/auth",
            routes![controllers::user::login, controllers::user::register,],
        )
        .mount(
            "/api/v1",
            routes![
                controllers::video::video_random,
                controllers::video::video_id,
            ],
        )
        .manage(db::init_pool())
        .manage(key)
        .launch();
}
