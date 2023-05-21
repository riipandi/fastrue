// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::prelude::*;

use crate::{entities, state};

#[handler]
pub async fn get_all(_req: &mut Request, res: &mut Response) {
    let data = sqlx::query_as::<_, entities::User>("select * from users")
        .fetch_all(state::dbconn())
        .await
        .unwrap();

    res.render(serde_json::to_string(&data).unwrap());
}
