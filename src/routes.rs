// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::oapi::{swagger_ui, Info, OpenApi};
use salvo::prelude::*;
use salvo::serve_static::static_embed;
use salvo::{catcher::Catcher, logging::Logger, proxy::Proxy};

use crate::handler::{error, root, user};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/web"]
struct Assets;

pub fn create_service() -> Service {
    let router = Router::new()
        .hoop(CatchPanic::new())
        .hoop(Logger::new())
        .get(root::hello)
        .push(api_routes())
        .push(health_check())
        .push(web_ui_route())
        .push(Router::with_path("500").get(error::error500));

    // Register Swagger OpenAPI endpoint
    let open_api_url = "/api-doc/openapi.json";
    let swagger_config = swagger_ui::Config::from(open_api_url)
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

    let doc = OpenApi::new(Info::new("Fastrue API", VERSION)).merge_router(&router);
    let swagger_route = swagger_ui::SwaggerUi::new(swagger_config).into_router("swagger");

    let router = router
        .push(doc.into_router(open_api_url))
        .push(swagger_route);

    Service::new(router).catcher(Catcher::default().hoop(error::error404))
}

fn api_routes() -> Router {
    Router::new()
        .path("api")
        .push(Router::with_path("users").get(user::get_all))
}

/**
 * Use proxied Vite React in development mode, and
 * use compiled Vite React SPA app in production mode.
 **/
fn web_ui_route() -> Router {
    let web_ui: Router = if cfg!(debug_assertions) {
        Router::with_path("ui/<**path>").handle(Proxy::new(vec!["http://localhost:3000/ui"]))
    } else {
        Router::with_path("ui/<**path>").get(static_embed::<Assets>().fallback("index.html"))
    };
    web_ui
}

fn health_check() -> Router {
    Router::new().path("health").get(root::health_check)
}
