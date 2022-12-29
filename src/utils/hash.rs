extern crate bcrypt;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn create_hash(plaintext: String) -> String {
    hash(plaintext, DEFAULT_COST).unwrap()
}

#[allow(dead_code)]
pub fn verify_hash(plaintext: String, hashed_str: String) -> bool {
    verify(plaintext, &hashed_str).unwrap()
}
