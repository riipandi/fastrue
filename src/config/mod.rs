use std::env;

pub mod app;
pub mod database;

// Set environment variable value
pub fn set_default_envar(key: &str, value: &str) {
    if env::var(key).is_err() {
        env::set_var(key, value);
    }
}
