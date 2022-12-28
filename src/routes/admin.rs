use axum::Router;

use crate::handler::admin;

pub fn register_admin_routes() -> Router {
    return Router::new()
        .merge(admin::invite::invite_by_admin())
        .merge(admin::admin::get_all_admin());
}
