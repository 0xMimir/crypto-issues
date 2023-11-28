use error::Result;
use sea_orm::{
    prelude::Uuid, DatabaseBackend, DatabaseConnection, EntityTrait, FromQueryResult, Statement,
};
use std::sync::Arc;

use super::super::contract::DbRepositoryContract;
use store::topics_repositories;

const UNSCRAPED_PROJECTS_QUERY: &str = include_str!("sql/unscraped_projects.sql");
const UNSCRAPED_REPOSITORIES_QUERY: &str = include_str!("sql/unscraped_repositories.sql");

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
    async fn get_unscraped_projects(&self) -> Result<Vec<String>> {
        let projects = topics_repositories::Entity::find()
            .from_raw_sql(Statement::from_string(
                DatabaseBackend::Postgres,
                UNSCRAPED_PROJECTS_QUERY,
            ))
            .into_model::<RepositoryOwner>()
            .all(self.conn.as_ref())
            .await?
            .into_iter()
            .map(|repo| repo.project_name)
            .collect();

        Ok(projects)
    }

    async fn get_unscraped_repositories(&self) -> Result<Vec<(String, String, Uuid)>> {
        let repositories = topics_repositories::Entity::find()
            .from_raw_sql(Statement::from_string(
                DatabaseBackend::Postgres,
                UNSCRAPED_REPOSITORIES_QUERY,
            ))
            .into_model::<Repository>()
            .all(self.conn.as_ref())
            .await?
            .into_iter()
            .map(|repo| (repo.project_name, repo.repository_name, repo.repository_owner))
            .collect();

        Ok(repositories)
    }
}

#[derive(FromQueryResult)]
pub struct RepositoryOwner {
    project_name: String,
}

#[derive(FromQueryResult)]
pub struct Repository {
    project_name: String,
    repository_name: String,
    repository_owner: Uuid,
}
