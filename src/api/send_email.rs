use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;

use super::route;
use crate::utils;

pub fn get_send_email() -> Router {
    async fn handler() -> impl IntoResponse {
        // Email content variables
        let from = "Admin Service <noreply@domain.tld>";
        let to = "Aris Ripandi <aris@domain.tld>";
        let subject = "Happy new year";
        let body = "Don't worry, be happy".to_string();

        match utils::sendmail::send_email_smtp(from, to, subject, body).await {
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

    route("/api/sendmail", get(handler))
}
