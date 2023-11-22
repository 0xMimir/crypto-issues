use error::Result;
use store::github_projects;

#[async_trait]
pub trait DbRepositoryContract {
    ///
    /// Get github projects from db
    ///
    async fn get_projects(&self) -> Result<Vec<github_projects::Model>>;
}

#[async_trait]
pub trait DbServiceContract {
    ///
    /// Update github project in db
    ///
    async fn update_project(&self, project: github_projects::ActiveModel) -> Result<()>;
}
