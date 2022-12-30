extern crate bcrypt;
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::{distributions::Alphanumeric, Rng};

pub fn generate_secret() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(40)
        .map(char::from)
        .collect()
}

pub fn create_hash(plaintext: String) -> String {
    hash(plaintext, DEFAULT_COST).unwrap()
}

#[allow(dead_code)]
pub fn verify_hash(plaintext: String, hashed_str: String) -> bool {
    verify(plaintext, &hashed_str).unwrap()
}
