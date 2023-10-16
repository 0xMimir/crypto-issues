use error::Result;
use sdks::github::data::GithubRepository;
use sea_orm::prelude::Uuid;
use store::github_projects::Model;

#[async_trait]
pub trait DbRepositoryContract {
    ///
    /// Get github projects from db
    ///
    async fn get_projects(&self) -> Result<Vec<Model>>;
}

#[async_trait]
pub trait DbServiceContract {
    ///
    /// Create entries in `github_repositories` table
    ///
    async fn create_repository(
        &self,
        project_id: Uuid,
        repositories: Vec<GithubRepository>,
    ) -> Result<()>;

    ///
    /// Delete github project
    ///
    async fn delete_project(&self, id: Uuid) -> Result<()>;
}
