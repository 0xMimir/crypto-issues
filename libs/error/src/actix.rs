use actix_web::{HttpResponse, ResponseError};

use crate::Error;

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        println!("{}", self);
        HttpResponse::Ok().finish()
    }
}
