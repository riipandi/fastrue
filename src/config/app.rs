use std::env;

use crate::utils;

pub fn bind_addr() -> String {
    // Read bind address from envar or set the default.
    utils::set_default_envar("BIND_PORT", "3030");
    utils::set_default_envar("BIND_ADDR", "127.0.0.1");
    let env_port = env::var("BIND_PORT").unwrap();
    let env_addr = env::var("BIND_ADDR").unwrap();
    let bind_addr = [env_addr, env_port].join(":");
    bind_addr
}
