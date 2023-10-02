use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use error::Result;
use sea_orm::prelude::Uuid;

use crate::api::cryptocurrencies::contract::CryptocurrenciesContract;

pub async fn get_issues<S: CryptocurrenciesContract>(
    path: Path<(String, Uuid)>,
    service: Data<S>,
) -> Result<HttpResponse> {
    let id = path.into_inner().1;
    let value = service.get_issues_for_repository(id).await?;
    Ok(HttpResponse::Ok().json(value))
}
