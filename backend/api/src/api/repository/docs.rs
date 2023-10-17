use super::handlers::*;
use store::objects::{GithubIssue, RepositoryView};
use support::pagination::{PaginatedGithubIssue, PaginatedRepositories};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_issues, get_repository_by_id, get_search),
    components(schemas(
        RepositoryView,
        PaginatedGithubIssue,
        GithubIssue,
        PaginatedRepositories
    ))
)]
pub struct RepositoryDocs;
