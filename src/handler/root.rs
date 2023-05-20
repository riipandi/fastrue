// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::prelude::*;

#[handler]
pub async fn health_check(_res: &mut Response) -> Result<&'static str, ()> {
    Ok("All is well")
}
