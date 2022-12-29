extern crate bcrypt;
use bcrypt::{hash, verify, DEFAULT_COST};

pub(crate) fn create_hash(plaintext: String) -> String {
    return hash(plaintext, DEFAULT_COST).unwrap();
}

#[allow(dead_code)]
pub(crate) fn verify_hash(plaintext: String, hashed_str: String) -> bool {
    return verify(plaintext, &hashed_str).unwrap();
}
