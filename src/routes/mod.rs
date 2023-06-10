// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod api;
pub mod root;
pub mod spa;

pub fn route(path: &str, method_router: axum::routing::MethodRouter<()>) -> axum::Router {
	axum::Router::new().route(path, method_router)
}
