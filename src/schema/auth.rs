// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtClaims {
	pub iss: String, // Issuer name (typically the app name)
	pub sub: String, // Subject (typically the client ID)
	pub uid: String, // User identifier (UUID)
	pub exp: i64,    // Expiration time (Unix timestamp)
	pub iat: i64,    // Issued at time (Unix timestamp)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
	pub email: String,
	pub password: String,
}
