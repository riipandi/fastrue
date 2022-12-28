/**
 * Swagger Open API configuration.
 * Get more example here: https://github.com/juhaku/utoipa/tree/master/examples/todo-axum
*/
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(), components(), modifiers(), tags())]
pub struct ApiDoc;
