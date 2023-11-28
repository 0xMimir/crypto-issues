use error::Result;
use sea_orm::{DatabaseConnection, EntityTrait, Statement, ConnectionTrait};
use std::sync::Arc;

use super::super::contract::DbServiceContract;
use store::{github_projects, github_repositories};

const UPDATE_STATEMENT: &str = include_str!("sql/update_statement.sql");

pub struct PgService {
    conn: Arc<DatabaseConnection>,
}

impl PgService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl DbServiceContract for PgService {
    async fn insert_project(&self, project: github_projects::ActiveModel) -> Result<()> {
        github_projects::Entity::insert(project)
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }

    async fn insert_repository(&self, repository: github_repositories::ActiveModel) -> Result<()> {
        github_repositories::Entity::insert(repository)
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }

    async fn update_projects(&self) -> Result<()> {
        self.conn
            .as_ref()
            .execute(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                UPDATE_STATEMENT,
            ))
            .await?;
        Ok(())
    }
}
