use std::env;

use error::{Error, Result};

pub fn get(key: &str) -> Result<String> {
    env::var(key).map_err(|_| Error::NotFoundWithCause(key.to_owned()))
}

pub fn dotenv_init() {
    dotenv::dotenv().expect("Error running reading dotenv");
}
