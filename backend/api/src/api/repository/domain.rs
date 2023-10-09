use super::{
    contract::{DbRepositoryContract, RepositoryContract},
    data::GetIssuesParams,
};
use error::Result;
use sea_orm::prelude::Uuid;
use store::{issues::Model as Issue, objects::RepositoryView};
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
    ) -> Result<Pagination<Issue>> {
        self.repository
            .get_issues_for_repository(repository_id, params)
            .await
    }
}
