use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use error::Result;
use validify::Validify;

use super::super::{
    contract::ProjectsContract, data::SearchGithubProjectParams,
    data::SearchGithubProjectParamsPayload,
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
    let params = SearchGithubProjectParams::validify(params.into_inner())?;
    let value = service.search(params).await?;
    Ok(HttpResponse::Ok().json(value))
}
