use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use chrono::{NaiveDateTime, Utc};
use reqwest::StatusCode;
use serde_json::Value;

use crate::Error;

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        let (message, cause) = self.get_message_and_cause();

        let value = ErrorResponse {
            status: status.as_u16(),
            message,
            cause,
            error: status.canonical_reason().unwrap_or_default().to_owned(),
            timestamp: Utc::now().naive_utc(),
        };

        HttpResponseBuilder::new(status).json(value)
    }

    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            Error::NotFound | Error::NotFoundWithCause(_) => StatusCode::NOT_FOUND,

            Error::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            Error::Validation(_) => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Error {
    /// Fix this
    fn get_message_and_cause(&self) -> (Value, Option<String>) {
        let message = match self {
            Self::Validation(error) => serde_json::to_value(error).expect("Bedja pusi k"),
            _ => Value::String(self.to_string()),
        };
        let cause = None;

        (message, cause)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    pub message: Value,
    pub cause: Option<String>,
    pub error: String,
    pub status: u16,
    pub timestamp: NaiveDateTime,
}
