use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

/**
 * Swagger Open API configuration.
 * Get more example here: https://github.com/juhaku/utoipa/tree/master/examples/todo-axum
*/

#[derive(OpenApi)]
#[openapi(paths(), components(), modifiers(), tags())]
pub struct ApiDoc;

pub fn register_swagger() -> SwaggerUi {
    let open_api_url = "/swagger/openapi.json";
    let config = Config::from(open_api_url)
        .display_request_duration(true)
        .try_it_out_enabled(true)
        .with_credentials(true)
        .persist_authorization(false);

    return SwaggerUi::new("/swagger")
        .url(open_api_url, ApiDoc::openapi())
        .config(config);
}
