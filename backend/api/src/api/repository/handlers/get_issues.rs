use crate::api::repository::data::GetIssuesParams;

use super::super::{contract::RepositoryContract, data::GetIssuesParamsPayload};
use actix_web::{
    web::{Data, Path, Query},
    HttpResponse,
};
use error::Result;
use validify::Validify;

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
            body = PaginatedGithubIssue
        )
    )
)]
pub async fn get_issues<S: RepositoryContract>(
    path: Path<(String, String)>,
    service: Data<S>,
    query: Query<GetIssuesParamsPayload>,
) -> Result<HttpResponse> {
    let repository_id = path.into_inner().1.parse()?;
    let query = GetIssuesParams::validify(query.into_inner())?;

    let value = service
        .get_issues_for_repository(repository_id, query)
        .await?;

    Ok(HttpResponse::Ok().json(value))
}
