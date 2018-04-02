use argon2::{Config, hash_encoded, verify_encoded};
use rand::{Rng, thread_rng};
use rocket::response::content::Json;
use rocket::response::Responder;
use rocket::request::Request;
use rocket::http::Status;

use serde::ser::Serialize;

pub fn hash_password(pw: &[u8]) -> argon2::Result<String> {
    let mut salt = [0u8; 10];
    thread_rng().fill(&mut salt[..]);
    hash_encoded(pw, &salt, &Config::default())
}

pub fn verify_password(hash: &str, pw: &[u8]) -> argon2::Result<bool> {
    verify_encoded(hash, pw)
}