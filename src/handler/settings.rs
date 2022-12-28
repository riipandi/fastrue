use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;

use crate::routes::route;

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
