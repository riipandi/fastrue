// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use sqlx::FromRow;
use tsync::tsync;
use uuid::Uuid;

#[tsync]
#[derive(Serialize, Deserialize, PartialEq, FromRow, Debug)]
#[allow(non_snake_case)]
pub struct User {
    #[serde(rename = "instanceId")]
    instance_id: Option<Uuid>,
    id: Uuid,
    aud: Option<String>,
    role: Option<String>,
    email: String,
    encrypted_password: Option<String>,
    confirmed_at: Option<DateTime<Utc>>,
    invited_at: Option<DateTime<Utc>>,
    confirmation_token: Option<String>,
    confirmation_sent_at: Option<DateTime<Utc>>,
    recovery_token: Option<String>,
    recovery_sent_at: Option<DateTime<Utc>>,
    email_change_token: Option<String>,
    email_change: Option<String>,
    email_change_sent_at: Option<DateTime<Utc>>,
    last_sign_in_at: Option<DateTime<Utc>>,
    raw_app_meta_data: Option<Json>,
    raw_user_meta_data: Option<Json>,
    #[serde(default = "default_is_super_admin")]
    is_super_admin: bool,
    #[serde(default = "current_timestamp")]
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
    #[serde(default = "current_timestamp")]
    #[serde(rename = "updatedAt")]
    updated_at: DateTime<Utc>,
}

fn default_is_super_admin() -> bool {
    false
}

fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}
