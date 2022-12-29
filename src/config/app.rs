use super::get_envar;

pub fn bind_addr() -> String {
    let env_addr = get_envar("BIND_ADDR", Some("127.0.0.1"));
    let env_port = get_envar("BIND_PORT", Some("3030"));
    [env_addr, env_port].join(":")
}
