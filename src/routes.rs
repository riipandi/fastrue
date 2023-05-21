// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::catcher::Catcher;
use salvo::logging::Logger;
use salvo::prelude::*;
use salvo::proxy::Proxy;
use salvo::serve_static::static_embed;

use crate::handler::{error, root, user};

#[derive(rust_embed::RustEmbed)]
#[folder = "web/"]
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
