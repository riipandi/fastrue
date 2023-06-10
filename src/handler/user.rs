// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn index() -> impl IntoResponse {
	let body = Json(json!({
	  "message": "Not yet implemented"
	}));
	(StatusCode::OK, body)
}

// use axum::{extract::State, http::StatusCode};
// use axum::{response::IntoResponse, Json};
// use sqlx::PgPool;

// use crate::schema::User;
// use crate::server::responder::JsonResponse;

// pub async fn index(State(pool): State<PgPool>) -> impl IntoResponse {
// 	let sql = "SELECT * FROM users".to_string();
// 	let result: Result<Vec<User>, sqlx::Error> =
// 		sqlx::query_as::<_, User>(&sql).fetch_all(&pool).await;

// 	match result {
// 		Ok(users) => (StatusCode::OK, Json(users)),
// 		Err(err) => {
// 			tracing::error!("error retrieving users: {:?}", err);
// 			(StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<User>::new()))
// 		}
// 	}

// 	// let users: Vec<User> = sqlx::query_as!(User, "select * from users")
// 	// 	.fetch_all(&pool)
// 	// 	.await
// 	// 	.expect("Unable to query users table");

// 	// let body: JsonResponse<Vec<User>> = JsonResponse {
// 	// 	status_code: StatusCode::OK.as_u16(),
// 	// 	message: Some("List all users"),
// 	// 	data: Some(users),
// 	// 	params: None,
// 	// };

// 	// (StatusCode::OK, Json(body))
// }
