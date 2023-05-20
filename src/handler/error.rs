// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::prelude::*;

struct ErrorResponse;

#[async_trait]
impl Writer for ErrorResponse {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render("custom error");
    }
}

#[handler]
pub async fn throw_response() -> Result<(), ErrorResponse> {
    Err(ErrorResponse)
}
