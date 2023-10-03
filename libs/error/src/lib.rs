use thiserror::Error;

mod actix;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Sea orm: {0}")]
    SeaOrm(#[from] sea_orm::error::DbErr),

    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde json: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Not found")]
    NotFound,

    #[error("Not found: {0}")]
    NotFoundWithCause(String),

    #[error("Internal server error: {0}")]
    InternalServer(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Validation error: {0}")]
    Validation(#[from] validify::ValidationErrors),

    #[error("Unauthorized")]
    Unauthorized
}

impl Error {
    pub fn add_cause<C: Into<String>>(self, cause: C) -> Self {
        match self {
            Self::NotFound | Self::NotFoundWithCause(_) => Self::NotFoundWithCause(cause.into()),
            _ => self,
        }
    }
}
