use error::Result;
use sea_orm::prelude::Uuid;
use store::objects::{GithubIssue, RepositoryView, SearchRepository};
use support::pagination::Pagination;

use super::data::{GetIssuesParams, SearchRepositoryParams};

#[async_trait]
pub trait DbRepositoryContract {
    ///
    /// Get paginated list of issues for repository
    ///
    async fn get_issues_for_repository(
        &self,
        repository_id: Uuid,
        params: GetIssuesParams,
    ) -> Result<Pagination<GithubIssue>>;

    ///
    /// Get repository view
    ///
    async fn get_repository(&self, id: Uuid) -> Result<RepositoryView>;

    ///
    /// Search over repositories
    ///
    async fn search_repositories(
        &self,
        params: SearchRepositoryParams,
    ) -> Result<Pagination<SearchRepository>>;
}

#[async_trait]
pub trait RepositoryContract {
    ///
    /// Get paginated list of issues for repository
    ///
    async fn get_issues_for_repository(
        &self,
        repository_id: Uuid,
        params: GetIssuesParams,
    ) -> Result<Pagination<GithubIssue>>;

    ///
    /// Get repository view
    ///
    async fn get_repository(&self, id: Uuid) -> Result<RepositoryView>;

    ///
    /// Search over repositories
    ///
    async fn search_repositories(
        &self,
        params: SearchRepositoryParams,
    ) -> Result<Pagination<SearchRepository>>;
}
