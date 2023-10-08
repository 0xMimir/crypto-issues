use std::env;

use error::{Error, Result};

///
/// Get variable from env
/// 
pub fn get(key: &str) -> Result<String> {
    env::var(key).map_err(|_| Error::NotFoundWithCause(key.to_owned()))
}

///
/// Load env variables from .env file
/// if file is not found not variables will be loaded
/// 
pub fn dotenv_init() {
    let _ = dotenv::dotenv();
}

///
/// Get variable from env if it doesn't exist
/// take default variable
/// 
pub fn get_default(key: &str, default: &str) -> String {
    get(key).unwrap_or(default.to_owned())
}
