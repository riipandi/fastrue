// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

// Original code taken from rust-embed example:
// https://github.com/pyrossh/rust-embed/tree/master/examples/axum-spa

use axum::body::{boxed, Full};
use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::Router;

static INDEX_HTML: &str = "index.html";

#[derive(rust_embed::RustEmbed)]
#[folder = "target/web/"]
struct Assets;

pub fn register() -> Router {
	Router::new().fallback(spa_handler)
}

async fn spa_handler(uri: Uri) -> impl IntoResponse {
	let path = uri.path().trim_start_matches('/');

	if path.is_empty() || path == INDEX_HTML {
		return index_spa().await;
	}

	match Assets::get(path) {
		Some(content) => {
			let body = boxed(Full::from(content.data));
			let mime = mime_guess::from_path(path).first_or_octet_stream();

			Response::builder()
				.header(header::CONTENT_TYPE, mime.as_ref())
				.body(body)
				.unwrap()
		}
		None => {
			if path.contains('.') {
				return not_found_spa().await;
			}
			index_spa().await
		}
	}
}

async fn index_spa() -> Response {
	match Assets::get(INDEX_HTML) {
		Some(content) => {
			let body = boxed(Full::from(content.data));

			Response::builder()
				.header(header::CONTENT_TYPE, "text/html")
				.body(body)
				.unwrap()
		}
		None => not_found_spa().await,
	}
}

async fn not_found_spa() -> Response {
	Response::builder()
		.status(StatusCode::NOT_FOUND)
		.body(boxed(Full::from("404")))
		.unwrap()
}
