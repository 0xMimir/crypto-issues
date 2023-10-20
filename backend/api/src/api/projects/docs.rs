use super::handlers::*;
use store::objects::SearchGithubProject;
use support::pagination::PaginatedProjects;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_search),
    components(schemas(PaginatedProjects, SearchGithubProject))
)]
pub struct ProjectsDocs;
