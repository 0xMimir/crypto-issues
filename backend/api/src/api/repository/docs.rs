use super::handlers::*;
use store::objects::{GithubIssue, RepositoryView};
use support::pagination::PaginatedGithubIssue;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_issues, get_repository_by_id),
    components(schemas(RepositoryView, PaginatedGithubIssue, GithubIssue))
)]
pub struct RepositoryDocs;
