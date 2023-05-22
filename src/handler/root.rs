// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::prelude::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[handler]
pub async fn hello(_res: &mut Response) -> Result<String, ()> {
    let build_timestamp = build_time::build_time_utc!();
    Ok(format!("Fastrue v{} ({})", VERSION, build_timestamp))
}

#[handler]
pub async fn health_check(_res: &mut Response) -> Result<&'static str, ()> {
    Ok("All is well")
}
