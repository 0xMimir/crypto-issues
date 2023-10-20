use actix_web::web::Query;
use actix_web::{web::Data, HttpResponse};
use error::Result;
use validify::Validate;

use super::super::{
    contract::ProjectsContract,
    data::{SearchGithubProjectParams, SearchGithubProjectParamsPayload},
};

#[utoipa::path(
    get,
    path = "/api/v1/projects",
    params(SearchGithubProjectParams),
    responses(
        (
            status = 200,
            description = "Search github projects",
            body = PaginatedProjects
        )
    )
)]
pub async fn get_search<S: ProjectsContract>(
    service: Data<S>,
    params: Query<SearchGithubProjectParamsPayload>,
) -> Result<HttpResponse> {
    params.validate()?;

    let params = params.into_inner().into();
    let value = service.search(params).await?;
    Ok(HttpResponse::Ok().json(value))
}
