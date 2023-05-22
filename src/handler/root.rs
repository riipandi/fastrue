// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use build_time::build_time_utc;
use salvo::prelude::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[handler]
pub async fn hello(_res: &mut Response) -> Result<String, ()> {
    Ok(format!("Fastrue v{} ({}).", VERSION, build_time_utc!()))
}

#[handler]
pub async fn health_check(_res: &mut Response) -> Result<&'static str, ()> {
    Ok("All is well")
}
