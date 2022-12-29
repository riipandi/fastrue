use axum::Router;

use crate::handler::{get_all_admin, invite_by_admin};

pub fn register_admin_routes() -> Router {
    Router::new()
        .merge(invite_by_admin())
        .merge(get_all_admin())
}
