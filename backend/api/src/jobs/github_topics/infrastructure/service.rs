use error::Result;
use sea_orm::{sea_query::OnConflict, ConnectionTrait, DatabaseConnection, EntityTrait, Statement};
use std::sync::Arc;

use super::super::contract::DbServiceContract;
use store::topics_repositories;

const UPDATE_STATEMENT: &str = include_str!("update_statement.sql");

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
    async fn upsert_projects(&self, projects: Vec<topics_repositories::ActiveModel>) -> Result<()> {
        let mut conflict = OnConflict::columns([
            topics_repositories::Column::RepositoryName,
            topics_repositories::Column::RepositoryOwner,
        ]);

        conflict.update_columns([topics_repositories::Column::StargazersCount]);

        topics_repositories::Entity::insert_many(projects)
            .on_conflict(conflict)
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
