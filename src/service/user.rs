use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// TODO: Remove Debug flag in production release!
#[derive(Serialize, Deserialize, PartialEq, FromRow, Debug)]
pub struct User {
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
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}
