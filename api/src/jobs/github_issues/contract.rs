use error::Result;
use sdks::github::data::GithubIssue;
use sea_orm::prelude::Uuid;
use store::{
    github_projects::Model as GithubProject, github_repositories::Model as GithubRepository,
};

#[async_trait]
pub trait DbRepositoryContract {
    ///
    /// Returns all github projects in db, might needed to be refactored to paginate
    ///
    async fn get_projects(&self) -> Result<Vec<GithubProject>>;

    ///
    /// Return all repositories for project, might needed to be refactored to paginate
    ///
    async fn get_project_repositories(&self, project_id: Uuid) -> Result<Vec<GithubRepository>>;
}

#[async_trait]
pub trait DbServiceContract {
    ///
    /// Create entities in `issues` table
    ///
    async fn create_issues(&self, repository_id: Uuid, issues: Vec<GithubIssue>) -> Result<()>;
}
