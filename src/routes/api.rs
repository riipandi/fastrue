// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::*, Json, Router};

use crate::server::responder::{self, JsonResponse};
use crate::{routes, server::middleware};

pub fn register() -> Router {
	Router::new()
		.layer(middleware::cors())
		.merge(api_root())
		.nest("/auth", auth_routes::register())
		.nest("/users", user_routes::register())
		.fallback(responder::throw_not_found)
}

pub fn api_root() -> Router {
	async fn handler() -> impl IntoResponse {
		let http_status = StatusCode::OK;
		let body: JsonResponse<&str> = JsonResponse {
			status_code: http_status.as_u16(),
			message: Some(format!("This is default {} API endpoint", crate::PKG_NAME)),
			data: None,
			params: None,
		};
		(http_status, Json(body))
	}
	routes::route("/", get(handler))
}

mod auth_routes {
	use crate::{handler::auth as handler, routes::route};
	use axum::{routing::*, Router};

	pub fn register() -> Router {
		Router::new()
			.merge(route("/login", post(handler::login)))
			.merge(route("/register", post(handler::signup)))
	}
}

mod user_routes {
	use crate::{handler::user as handler, routes::route};
	use axum::{routing::*, Router};

	pub fn register() -> Router {
		Router::new().merge(route("/", get(handler::index)))
	}
}
