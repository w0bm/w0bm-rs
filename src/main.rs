#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

mod schema;
mod models;
mod controllers;
mod db;
mod util;


fn main() {
    rocket::ignite()
        .mount("/",
               routes![])
        .manage(db::init_pool())
        .launch();
}
