use axum::Router;

use crate::handler::{logout, recover, signup, token, user, verify};

pub fn register_auth_routes() -> Router {
    return Router::new()
        .merge(signup::signup())
        .merge(verify::verify())
        .merge(token::token())
        .merge(user::user())
        .merge(logout::logout())
        .merge(recover::recover());
}
