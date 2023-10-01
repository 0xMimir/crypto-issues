use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Sea orm: {0}")]
    SeaOrm(#[from] sea_orm::error::DbErr),

    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde json: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalServer(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded
}
