use argon2::{self, Config, hash_encoded, verify_encoded};
use ring::rand::*;
use std::ops::Deref;

pub fn hash_password(pw: &[u8]) -> argon2::Result<String> {
    let mut salt = [0u8; 10];
    SystemRandom::new().fill(&mut salt).map_err(|_| argon2::Error::DecodingFail)?;
    hash_encoded(pw, &salt, &Config::default())
}

pub fn verify_password(hash: &str, pw: &[u8]) -> bool {
    verify_encoded(hash, pw).unwrap_or(false)
}

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