use error::Result;
use sea_orm::prelude::Uuid;
use store::{github_projects, github_repositories};

#[async_trait]
pub trait DbServiceContract {
    ///
    /// Insert new project
    /// 
    async fn insert_project(&self, project: github_projects::ActiveModel) -> Result<()>;

    ///
    /// Insert new repository
    /// 
    async fn insert_repository(&self, repository: github_repositories::ActiveModel) -> Result<()>;

    ///
    /// Run update status of github topic repositories
    /// 
    async fn update_projects(&self) -> Result<()>;
}

#[async_trait]
pub trait DbRepositoryContract {
    ///
    /// Get all projects from github topic repositories that are not scraped
    /// 
    async fn get_unscraped_projects(&self) -> Result<Vec<String>>;

    ///
    /// Get unscraped github topic repositories, return (Project Name, Repository Name, Project ID)
    /// 
    async fn get_unscraped_repositories(&self) -> Result<Vec<(String, String, Uuid)>>;
}
