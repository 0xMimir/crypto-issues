use std::env;

use error::{Error, Result};

pub fn get(key: &str) -> Result<String> {
    env::var(key).map_err(|_| Error::NotFoundWithCause(key.to_owned()))
}

pub fn dotenv_init() {
    let _ = dotenv::dotenv();
}

pub fn get_default(key: &str, default: &str) -> String {
    get(key).unwrap_or(default.to_owned())
}
