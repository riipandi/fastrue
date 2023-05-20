// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

use crate::handler::{admin, appinfo, auth, user};

#[derive(OpenApi)]
#[openapi(
    paths(
        admin::get_all_users,
        admin::invite_by_admin,
        appinfo::health_check,
        appinfo::settings,
        auth::logout,
        auth::recover,
        auth::token,
        auth::signup,
        auth::verify,
        user::get_user,
        user::get_user,
        user::put_user,
        user::put_user,
    ),
    components(),
    modifiers(),
    tags()
)]
pub struct ApiDoc;

pub fn register_swagger() -> SwaggerUi {
    let open_api_url = "/swagger/openapi.json";
    let config = Config::from(open_api_url)
        .doc_expansion("list")
        .display_request_duration(true)
        .deep_linking(false)
        .filter(false)
        .try_it_out_enabled(false)
        .request_snippets_enabled(false)
        .show_mutated_request(false)
        .with_credentials(true)
        .persist_authorization(false)
        .use_base_layout();

    SwaggerUi::new("/swagger")
        .url(open_api_url, ApiDoc::openapi())
        .config(config)
}
