use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use error::Result;

use super::super::contract::CryptocurrenciesContract;

#[utoipa::path(
    get,
    path = "/api/v1/crypto/{id}",
    params(
        ("id", description = "Crypto id")
    ),
    responses(
        (
            status = 200,
            description = "Crypto currency data with all full data for repositories",
            body = CryptoCurrencyWithRepositories
        )
    )
)]
pub async fn get_cryptocurrency_by_id<S: CryptocurrenciesContract>(
    path: Path<(String, String)>,
    service: Data<S>,
) -> Result<HttpResponse> {
    let id = path.into_inner().1.parse()?;
    let value = service.get_cryptocurrency(id).await?;
    Ok(HttpResponse::Ok().json(value))
}
