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
extern crate rand;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod schema;
mod models;
mod controllers;
mod db;
mod util;


fn main() {
    rocket::ignite()
        .mount("/api/v1",
               routes![])
        .manage(db::init_pool())
        .launch();
}
