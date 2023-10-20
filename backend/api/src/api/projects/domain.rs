use super::{
    contract::{DbRepositoryContract, ProjectsContract},
    data::SearchGithubProjectParams,
};
use error::Result;
use store::objects::SearchGithubProject;
use support::pagination::Pagination;

pub struct Projects<Repository: DbRepositoryContract> {
    repository: Repository,
}

impl<Repository> Projects<Repository>
where
    Repository: DbRepositoryContract + Send + Sync,
{
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<Repository> ProjectsContract for Projects<Repository>
where
    Repository: DbRepositoryContract + Send + Sync,
{
    async fn search(
        &self,
        params: SearchGithubProjectParams,
    ) -> Result<Pagination<SearchGithubProject>> {
        self.repository.search(params).await
    }
}
