// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use axum::{
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde_json::json;

use crate::routes::route;

#[utoipa::path(
    post,
    path = "/api/verify",
    tag = "Authentication",
    responses(
        (status = 200, description = "Verify email address")
    ),
)]
pub fn verify() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/verify", post(handler))
}
