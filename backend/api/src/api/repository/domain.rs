use super::{
    contract::{DbRepositoryContract, RepositoryContract},
    data::{GetIssuesParams, SearchRepositoryParams},
};
use error::Result;
use sea_orm::prelude::Uuid;
use store::objects::{GithubIssue, RepositoryView, SearchRepository};
use support::pagination::Pagination;

pub struct Repository<A: DbRepositoryContract> {
    pub(super) repository: A,
}

#[async_trait]
impl<A: DbRepositoryContract + Send + Sync> RepositoryContract for Repository<A> {
    async fn get_repository(&self, id: Uuid) -> Result<RepositoryView> {
        self.repository.get_repository(id).await
    }

    async fn get_issues_for_repository(
        &self,
        repository_id: Uuid,
        params: GetIssuesParams,
    ) -> Result<Pagination<GithubIssue>> {
        self.repository
            .get_issues_for_repository(repository_id, params)
            .await
    }

    async fn search_repositories(
        &self,
        params: SearchRepositoryParams,
    ) -> Result<Pagination<SearchRepository>> {
        self.repository.search_repositories(params).await
    }
}
