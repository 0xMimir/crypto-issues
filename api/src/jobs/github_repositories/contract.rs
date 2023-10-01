use error::Result;
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
    async fn create_repository(&self, project_id: Uuid, repositories: Vec<String>) -> Result<()>;
}
