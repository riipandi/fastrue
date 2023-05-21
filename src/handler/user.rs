// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::http::StatusCode;

use salvo::prelude::*;
use serde::Serialize;

use crate::{entities, state};

#[derive(Serialize, Debug)]
struct UserResponse {
    status: String,
    data: Vec<entities::User>,
}

/// This is a summary of the operation
///
/// All lines of the doc comment will be included to operation description.
#[endpoint(
    responses(
        (status = 200, description = "Get all users"),
        (status = 401, description = "Unauthorized to delete resource"),
        (status = 404, description = "Resource not found")
    ),
)]
pub async fn get_all(_req: &mut Request, res: &mut Response) {
    let data = sqlx::query_as::<_, entities::User>("select * from users")
        .fetch_all(state::dbconn())
        .await
        .unwrap();

    let result = UserResponse {
        status: "success".to_string(),
        data,
    };

    res.status_code(StatusCode::OK);
    res.render(Json(result));
}
