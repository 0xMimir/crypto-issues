use super::handlers::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_issues, get_repository_by_id))]
pub struct RepositoryDocs;
