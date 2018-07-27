use argon2::{hash_encoded, verify_encoded, Config};
use rand::distributions::{Range, Sample};
use ring::rand::*;
use slug::slugify;
use std::ops::Deref;
use failure::Error;

pub fn hash_password(pw: &[u8]) -> Result<String, Error> {
    let mut salt = [0u8; 10];
    SystemRandom::new()
        .fill(&mut salt)?;
    hash_encoded(pw, &salt, &Config::default()).map_err(From::from)
}

pub fn verify_password(hash: &str, pw: &[u8]) -> bool {
    verify_encoded(hash, pw).map_err(|e| error!("{}", e)).unwrap_or(false)
}

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct Secret([u8; 32]);

impl Deref for Secret {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn generate_secret() -> Result<Secret, ::ring::error::Unspecified> {
    let mut buf = [0u8; 32];
    SystemRandom::new().fill(&mut buf)?;
    Ok(Secret(buf))
}

pub fn rand_range(lower: i64, upper: i64) -> i64 {
    let mut rng = ::rand::thread_rng();
    Range::new(lower, upper).sample(&mut rng)
}

pub fn normalize<S: AsRef<str>>(s: S) -> String {
    slugify(s).replace('-', "")
}
