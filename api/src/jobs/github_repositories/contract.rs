use error::Result;

#[async_trait]
pub trait DbRepositoryContract {
    ///
    /// Get github projects from db
    ///
    async fn get_projects(&self) -> Result<Vec<String>>;
}

#[async_trait]
pub trait DbServiceContract {
    ///
    /// Create entries in `github_repositories` table
    ///
    async fn create_repository(&self, project: String, repository: Vec<String>) -> Result<()>;
}
