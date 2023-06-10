// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod cmd;
pub mod handler;
pub mod routes;
pub mod schema;
pub mod server;
pub mod utils;

pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const PKG_ARCH: &'static str = std::env::consts::ARCH;
pub const PKG_OS: &'static str = std::env::consts::OS;
pub const BUILD_TIME: &'static str = build_time::build_time_utc!("%Y-%m-%d %H:%M:%S UTC");
