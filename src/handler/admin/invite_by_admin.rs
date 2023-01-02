use axum::{
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde_json::json;

use crate::{routes::route, utils};

#[utoipa::path(
    post,
    path = "/api/invite",
    tag = "Administration",
    responses((status = 200, description = "Invite user by admin")),
    params(
      ("email" = String, Path, description = "Pet database id to get Pet for"),
      ("x-csrf-token", Header, description = "CSRF token in header", deprecated)
    ),
)]
pub fn invite_by_admin() -> Router {
    async fn handler() -> impl IntoResponse {
        // Email content variables
        let from = "Admin Service <noreply@domain.tld>";
        let to = "Aris Ripandi <aris@domain.tld>";
        let subject = "Happy new year";
        let body = "Don't worry, be happy".to_string();

        match utils::mailer::send_email_smtp(from, to, subject, body).await {
            Ok(_) => {
                tracing::info!("Email for {:?} was sent", to);
                Json(json!({
                  "message": "Email sent successfully!",
                  "code": 200,
                }))
            }
            Err(e) => {
                tracing::error!("Could not send email: {:?}", e);
                Json(json!({
                  "message": "Could not send email",
                  "code": 500,
                }))
            }
        }
    }
    route("/invite", post(handler))
}
