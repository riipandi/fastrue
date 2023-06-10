// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
	pub id: Uuid,
	pub email: String,
	pub name: String,
	#[serde(rename = "passwordHash")]
	pub password_hash: String,
	#[serde(default = "current_timestamp")]
	#[serde(rename = "createdAt")]
	pub created_at: DateTime<Utc>,
	#[serde(default = "current_timestamp")]
	#[serde(rename = "updatedAt")]
	pub updated_at: DateTime<Utc>,
}

fn current_timestamp() -> DateTime<Utc> {
	Utc::now()
}
