use argon2::{self, Config, hash_encoded, verify_encoded};
use rand::{Rng, thread_rng};

pub fn hash_password(pw: &[u8]) -> argon2::Result<String> {
    let salt: [u8; 10] = thread_rng().gen();
    hash_encoded(pw, &salt, &Config::default())
}

pub fn verify_password(hash: &str, pw: &[u8]) -> argon2::Result<bool> {
    verify_encoded(hash, pw)
}
