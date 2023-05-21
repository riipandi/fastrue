// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// TODO: Remove Debug flag in production release!
#[derive(Serialize, Deserialize, PartialEq, FromRow, Debug)]
#[allow(non_snake_case)]
pub struct User {
    #[serde(rename = "instanceId")]
    instance_id: Option<Uuid>,
    id: Option<Uuid>,
    aud: Option<String>,
    role: Option<String>,
    email: Option<String>,
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
    raw_app_meta_data: Option<String>,
    raw_user_meta_data: Option<String>,
    is_super_admin: Option<bool>,
    #[serde(rename = "createdAt")]
    created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    updated_at: Option<DateTime<Utc>>,
}
