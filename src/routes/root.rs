// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

// use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use axum::{http::StatusCode, routing::*, Router};

use crate::server::BASE_PATH_API;
use crate::{routes, server::responder};

pub fn register() -> Router {
	Router::new()
		.merge(home_route())
		.merge(health_check())
		.fallback(responder::throw_not_found)
}

fn home_route() -> Router {
	async fn handler() -> impl IntoResponse {
		Redirect::to(BASE_PATH_API)
	}
	routes::route("/", get(handler))
}

fn health_check() -> Router {
	async fn handler() -> impl IntoResponse {
		tracing::info!("Health check success");
		responder::as_plain(StatusCode::OK, crate::cmd::about())

		// let uri = crate::utils::get_envar("DATABASE_URL", None);
		// let pool = Pool::<Postgres>::connect(&uri).await.unwrap();

		// let result = sqlx::query!("SELECT 1 as value").fetch_one(&pool);

		// match result {
		// 	Ok(users) => (StatusCode::OK, Json(users)),
		// 	Err(err) => {
		// 		tracing::error!("error retrieving users: {:?}", err);
		// 		(StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<User>::new()))
		// 	}
		// }
	}

	// async fn handler(State(pool): State<sqlx::PgPool>) -> Result<impl IntoResponse, anyhow::Error> {
	// 	// Make a simple query to check database connected or not
	// 	let result = sqlx::query!("SELECT 1 as value").fetch_one(&pool);

	// 	// anyhow::bail!("Health check error: {}", err);
	// 	match result.await {
	// 		Ok(_res) => {
	// 			tracing::info!("Health check success");
	// 			responder::as_plain(StatusCode::OK, crate::cmd::about())
	// 		}
	// 		Err(err) => anyhow::bail!("Health check error: {}", err),
	// 	}
	// }

	routes::route("/health", get(handler))
}
