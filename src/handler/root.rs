// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::prelude::*;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[handler]
pub async fn hello(_res: &mut Response) -> Result<String, ()> {
    Ok(format!(
        "{} {} {} ({})",
        PKG_NAME,
        PKG_VERSION,
        format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS),
        build_time::build_time_utc!("%Y-%m-%d %H:%M:%S UTC")
    ))
}

#[handler]
pub async fn health_check(_res: &mut Response) -> Result<&'static str, ()> {
    Ok("All is well")
}
