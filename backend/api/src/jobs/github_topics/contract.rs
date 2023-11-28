use error::Result;
use store::topics_repositories;

#[async_trait]
pub trait DbServiceContract {
    ///
    /// Update github project in db
    ///
    async fn upsert_projects(&self, project: Vec<topics_repositories::ActiveModel>) -> Result<()>;

    ///
    /// Update scraped projects
    ///
    async fn update_projects(&self) -> Result<()>;
}
