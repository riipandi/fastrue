// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use indicatif::{style::TemplateError, ProgressStyle};
use std::env;

pub fn bind_addr() -> String {
    let env_addr = get_envar("BIND_ADDR", Some("127.0.0.1"));
    let env_port = get_envar("BIND_PORT", Some("9999"));
    [env_addr, env_port].join(":")
}

pub fn progressbar_style() -> Result<ProgressStyle, TemplateError> {
    ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
    )
}

pub fn get_envar(key: &str, default: Option<&str>) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => match default {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
    }
}
