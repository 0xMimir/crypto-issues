use error::Result;
use sea_orm::prelude::Uuid;
use store::objects::{GithubIssue, RepositoryView};
use support::pagination::Pagination;

use super::data::GetIssuesParams;

#[async_trait]
pub trait DbRepositoryContract {
    async fn get_issues_for_repository(
        &self,
        repository_id: Uuid,
        params: GetIssuesParams,
    ) -> Result<Pagination<GithubIssue>>;
    async fn get_repository(&self, id: Uuid) -> Result<RepositoryView>;
}

#[async_trait]
pub trait RepositoryContract {
    async fn get_issues_for_repository(
        &self,
        repository_id: Uuid,
        params: GetIssuesParams,
    ) -> Result<Pagination<GithubIssue>>;
    async fn get_repository(&self, id: Uuid) -> Result<RepositoryView>;
}
