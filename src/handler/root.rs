// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

// The file `built.rs` was placed there by cargo and `build.rs`
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use salvo::prelude::*;

#[handler]
pub async fn hello(_res: &mut Response) -> Result<String, ()> {
    Ok(format!(
        "Fastrue v{}, built for {} ({}).\n",
        built_info::PKG_VERSION,
        built_info::TARGET,
        built::util::strptime(built_info::BUILT_TIME_UTC)
    ))
}

#[handler]
pub async fn health_check(_res: &mut Response) -> Result<&'static str, ()> {
    Ok("All is well")
}
