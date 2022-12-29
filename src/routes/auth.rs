use axum::Router;

use crate::handler::{logout, recover, signup, token, user, verify};

pub fn register_auth_routes() -> Router {
    Router::new()
        .merge(signup::signup())
        .merge(verify::verify())
        .merge(token::post_token())
        .merge(user::get_user())
        .merge(user::put_user())
        .merge(logout::post_logout())
        .merge(recover::post_recover())
}
