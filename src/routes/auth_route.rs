use axum::Router;

use crate::handler::{get_all_admin, invite_by_admin, settings};
use crate::handler::{logout, recover, signup, token, user, verify};

pub fn register_auth_routes() -> Router {
    Router::new()
        .merge(settings::settings())
        .merge(signup::signup())
        .merge(verify::verify())
        .merge(token::post_token())
        .merge(user::get_user())
        .merge(user::put_user())
        .merge(logout::post_logout())
        .merge(recover::post_recover())
        .merge(invite_by_admin())
        .merge(get_all_admin())
}
