// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::net::{Ipv4Addr, Ipv6Addr};

pub fn get_envar(key: &str, default: Option<&str>) -> String {
	match std::env::var(key) {
		Ok(val) => val,
		Err(_) => match default {
			Some(val) => val.to_string(),
			None => "".to_string(),
		},
	}
}

pub fn generate_uid() -> String {
	nano_id::gen!(uid, 36, b"0123456789abcdefghijklmnopqrstuvwxyz");
	uid::<20>()
}

pub fn is_valid_email(email: &str) -> bool {
	let parts: Vec<&str> = email.split('@').collect();
	if parts.len() != 2 {
		return false;
	}

	let local = parts[0];
	let domain = parts[1];

	// Check that the local part is not empty
	if local.is_empty() {
		return false;
	}

	// Check that the domain part is not empty
	if domain.is_empty() {
		return false;
	}

	// Check that the domain part is a valid domain name
	match domain.parse::<Ipv4Addr>() {
		Ok(_) => true,
		Err(_) => match domain.parse::<Ipv6Addr>() {
			Ok(_) => true,
			Err(_) => {
				let domain_parts: Vec<&str> = domain.split('.').collect();
				if domain_parts.len() < 2 {
					return false;
				}
				domain_parts.iter().all(|part| !part.is_empty())
			}
		},
	}
}
