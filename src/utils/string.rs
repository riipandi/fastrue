// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

extern crate bcrypt;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn generate_secret() -> String {
    std::iter::repeat_with(fastrand::alphanumeric)
        .take(40)
        .collect()
}

// pub fn random_number_between(lower_bound: i32, upper_bound: i32) -> i32 {
//     fastrand::i32(lower_bound..upper_bound)
// }

pub fn create_hash(plaintext: String) -> String {
    hash(plaintext, DEFAULT_COST).unwrap()
}

#[allow(dead_code)]
pub fn verify_hash(plaintext: String, hashed_str: String) -> bool {
    verify(plaintext, &hashed_str).unwrap()
}
