use std::env;

use super::set_default_envar;

pub fn bind_addr() -> String {
    // Read bind address from envar or set the default.
    set_default_envar("BIND_PORT", "3030");
    set_default_envar("BIND_ADDR", "127.0.0.1");
    let env_port = env::var("BIND_PORT").unwrap();
    let env_addr = env::var("BIND_ADDR").unwrap();

    [env_addr, env_port].join(":")
}
