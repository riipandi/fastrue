// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::{http::StatusCode, prelude::*};

use crate::{entities, service, utils};

#[derive(serde::Serialize, Debug)]
struct JsonResponse<T> {
    status_code: i16,
    data: Vec<T>,
}

/// This is a summary of the operation
///
/// All lines of the doc comment will be included to operation description.
#[endpoint(
    responses(
        (status = 200, description = "Get all users"),
        (status = 401, description = "Unauthorized to access resource"),
        (status = 404, description = "Resource not found")
    ),
)]
pub async fn get_all(_req: &mut Request, res: &mut Response) {
    let data: Vec<entities::User> = service::user::get_all()
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch users: {}", err);
            res.status_code(StatusCode::BAD_REQUEST);
        })
        .unwrap();

    let status = StatusCode::OK;
    let result = JsonResponse {
        status_code: utils::get_status_code(status),
        data,
    };

    res.status_code(status);
    res.render(Json(result));
}
