// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::prelude::*;
use serde::Serialize;

use crate::utils;

struct ErrorResponse;

#[derive(Serialize, Debug)]
struct ResponseError {
    status_code: i16,
    message: String,
}

#[async_trait]
impl Writer for ErrorResponse {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        res.status_code(status);
        res.render(Json(ResponseError {
            status_code: utils::get_status_code(status),
            message: "Unhandled execption".to_string(),
        }));
    }
}

#[handler]
pub async fn error500() -> Result<(), ErrorResponse> {
    Err(ErrorResponse)
}

#[handler]
pub async fn error404(
    &self,
    _req: &Request,
    _depot: &Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        let status = StatusCode::NOT_FOUND;
        res.status_code(status);
        res.render(Json(ResponseError {
            status_code: utils::get_status_code(status),
            message: "Resource not found".to_string(),
        }));
        ctrl.skip_rest();
    }
}

#[handler]
pub async fn error_headless(res: &mut Response) {
    let status = StatusCode::NOT_FOUND;
    res.status_code(status);
    res.render(Json(ResponseError {
        status_code: utils::get_status_code(status),
        message: "Resource not found".to_string(),
    }));
}
