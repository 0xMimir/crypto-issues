use error::Result;
use sea_orm::{prelude::Uuid, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use std::sync::Arc;

use super::super::contract::DbRepositoryContract;
use store::{
    github_projects::Model as GithubProject,
    github_repositories::{Column, Model as GithubRepository},
    prelude::{GithubProjects, GithubRepositories},
};
pub struct PgRepository {
    conn: Arc<DatabaseConnection>,
}

impl PgRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl DbRepositoryContract for PgRepository {
    async fn get_projects(&self) -> Result<Vec<GithubProject>> {
        let projects = GithubProjects::find().all(self.conn.as_ref()).await?;
        Ok(projects)
    }
    async fn get_project_repositories(&self, project_id: Uuid) -> Result<Vec<GithubRepository>> {
        let repositories = GithubRepositories::find()
            .filter(Column::Project.eq(project_id))
            .all(self.conn.as_ref())
            .await?;

        Ok(repositories)
    }
}
