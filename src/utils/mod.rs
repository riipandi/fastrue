use std::env;

// Set environment variable value
pub fn set_default_envar(key: &str, value: &str) {
    if env::var(key).is_err() {
        env::set_var(key, value);
    }
}

pub mod api_helpers;
pub mod sendmail;
pub mod string;
pub mod throw_error;
