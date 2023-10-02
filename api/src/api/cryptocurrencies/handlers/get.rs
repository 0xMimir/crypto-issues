use actix_web::{web::Data, HttpResponse};
use error::Result;

use crate::api::cryptocurrencies::contract::CryptocurrenciesContract;

pub async fn get_cryptocurrencies<S: CryptocurrenciesContract>(
    service: Data<S>,
) -> Result<HttpResponse> {
    let value = service.get_cryptocurrencies().await?;
    Ok(HttpResponse::Ok().json(value))
}
