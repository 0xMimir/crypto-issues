use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use error::Result;
use validify::Validate;

use crate::api::cryptocurrencies::{
    contract::CryptocurrenciesContract, data::GetCryptoCurrenciesQueryPayload,
};

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
    query: Query<GetCryptoCurrenciesQueryPayload>,
) -> Result<HttpResponse> {
    query.validate()?;
    let query = query.into_inner().into();

    let value = service
        .get_cryptocurrencies(query)
        .await?;

    Ok(HttpResponse::Ok().json(value))
}
