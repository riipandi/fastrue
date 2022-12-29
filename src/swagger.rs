use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

use crate::handler::{
    admin::{admin, invite},
    health, logout, recover, settings, signup, token, user, verify,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        settings::settings,
        health::health_check,
        user::get_user,
        user::put_user,
        signup::signup,
        verify::verify,
        token::post_token,
        user::get_user,
        user::put_user,
        logout::post_logout,
        recover::post_recover,
        admin::get_all_admin,
        invite::invite_by_admin,
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

    return SwaggerUi::new("/swagger")
        .url(open_api_url, ApiDoc::openapi())
        .config(config);
}
