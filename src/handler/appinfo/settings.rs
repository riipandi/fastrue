// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use axum::response::{IntoResponse, Json};
use axum::{routing::get, Router};
use serde_json::json;

use crate::routes::route;

#[utoipa::path(
    get,
    path = "/api/settings",
    tag = "Information",
    responses(
        (status = 200, description = "Settings endpoint")
    ),
)]
pub fn settings() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "external": {
            "bitbucket": false,
            "github": true,
            "gitlab": false,
            "google": false,
            "facebook": false,
            "email": true,
            "saml": false
          },
          "external_labels": {},
          "disable_signup": false,
          "autoconfirm": false
        }))
    }
    route("/settings", get(handler))
}
