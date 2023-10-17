use actix_web::web::Query;
use actix_web::{web::Data, HttpResponse};
use error::Result;
use validify::Validate;

use super::super::data::{SearchRepositoryParams, SearchRepositoryParamsPayload};

use super::super::contract::RepositoryContract;

#[utoipa::path(
    get,
    path = "/api/v1/repository/search",
    params(SearchRepositoryParams),
    responses(
        (
            status = 200,
            description = "Crypto currency data with all full data for repositories",
            body = PaginatedRepositories
        )
    )
)]
pub async fn get_search<S: RepositoryContract>(
    service: Data<S>,
    params: Query<SearchRepositoryParamsPayload>,
) -> Result<HttpResponse> {
    params.validate()?;

    let params = params.into_inner().into();
    let value = service.search_repositories(params).await?;
    Ok(HttpResponse::Ok().json(value))
}
