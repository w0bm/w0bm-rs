#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate chrono;
extern crate argon2;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate jsonwebtoken as jwt;
extern crate ring;

mod schema;
mod models;
mod controllers;
mod db;
mod util;


fn main() {
    let key = util::generate_secret().expect("Error generating random secret");
    rocket::ignite()
        .mount("/auth",
            routes![
                controllers::user::login,
                controllers::user::register,
            ])
        .mount("/api/v1",
               routes![])
        .manage(db::init_pool())
        .manage(key)
        .launch();
}
