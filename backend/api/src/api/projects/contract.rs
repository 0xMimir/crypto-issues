use error::Result;
use store::objects::SearchGithubProject;
use support::pagination::Pagination;

use super::data::SearchGithubProjectParams;

#[async_trait]
pub trait DbRepositoryContract {
    ///
    /// Search github projects with params
    ///
    async fn search(
        &self,
        params: SearchGithubProjectParams,
    ) -> Result<Pagination<SearchGithubProject>>;
}

#[async_trait]
pub trait ProjectsContract {
    ///
    /// Search github projects with params
    ///
    async fn search(
        &self,
        params: SearchGithubProjectParams,
    ) -> Result<Pagination<SearchGithubProject>>;
}
