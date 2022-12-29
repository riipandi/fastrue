use std::env;

pub mod app;
pub mod database;

pub use self::app::*;
pub use self::database::*;

pub fn get_envar(key: &str, default: Option<&str>) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => match default {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
    }
}
