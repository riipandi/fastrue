// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::prelude::*;
use serde::Serialize;

struct ErrorResponse;

#[derive(Serialize, Debug)]
struct ResponseError {
    status: String,
    message: String,
}

#[async_trait]
impl Writer for ErrorResponse {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(ResponseError {
            status: "error".to_string(),
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
        res.status_code(StatusCode::NOT_FOUND);
        res.render(Json(ResponseError {
            status: "error".to_string(),
            message: "Resource not found".to_string(),
        }));
        ctrl.skip_rest();
    }
}
