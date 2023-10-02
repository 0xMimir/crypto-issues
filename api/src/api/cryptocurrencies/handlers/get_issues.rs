use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use error::Result;
use sea_orm::prelude::Uuid;

use crate::api::cryptocurrencies::contract::CryptocurrenciesContract;

#[utoipa::path(
    get,
    path = "/api/v1/repository/{id}/issues",
    params(
        ("id", description = "Repository id")
    ),
    responses(
        (
            status = 200,
            description = "List of issues for repository",
            body = [Model]
        )
    )
)]
pub async fn get_issues<S: CryptocurrenciesContract>(
    path: Path<(String, Uuid)>,
    service: Data<S>,
) -> Result<HttpResponse> {
    let id = path.into_inner().1;
    let value = service.get_issues_for_repository(id).await?;
    Ok(HttpResponse::Ok().json(value))
}
