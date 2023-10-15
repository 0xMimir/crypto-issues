use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use error::Result;

use super::super::contract::RepositoryContract;

#[utoipa::path(
    get,
    path = "/api/v1/repository/{id}",
    params(
        ("id", description = "Repository id")
    ),
    responses(
        (
            status = 200,
            description = "Crypto currency data with all full data for repositories",
            body = RepositoryView
        )
    )
)]
pub async fn get_repository_by_id<S: RepositoryContract>(
    path: Path<(String, String)>,
    service: Data<S>,
) -> Result<HttpResponse> {
    let id = path.into_inner().1.parse()?;
    let value = service.get_repository(id).await?;
    Ok(HttpResponse::Ok().json(value))
}
