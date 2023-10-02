use actix_web::{web::Data, HttpResponse};
use error::Result;

use crate::api::cryptocurrencies::contract::CryptocurrenciesContract;

#[utoipa::path(
    get,
    path = "/api/v1/crypto",
    responses(
        (
            status = 200,
            description = "List of crypto currencies with available repositories and total issues",
            body = [CryptoCurrencyView]
        )
    )
)]
pub async fn get_cryptocurrencies<S: CryptocurrenciesContract>(
    service: Data<S>,
) -> Result<HttpResponse> {
    let value = service.get_cryptocurrencies().await?;
    Ok(HttpResponse::Ok().json(value))
}
