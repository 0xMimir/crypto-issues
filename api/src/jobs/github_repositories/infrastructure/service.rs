use error::Result;
use sea_orm::{sea_query::OnConflict, ActiveValue::Set, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use super::super::contract::DbServiceContract;
use store::github_repositories::{ActiveModel, Column, Entity};

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
    async fn create_repository(&self, project: String, repositories: Vec<String>) -> Result<()> {
        let models = repositories
            .into_iter()
            .map(|repository_name| ActiveModel {
                project: Set(project.to_owned()),
                repository_name: Set(repository_name),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        Entity::insert_many(models)
            .on_conflict(
                OnConflict::columns([Column::Project, Column::RepositoryName])
                    .do_nothing()
                    .to_owned(),
            )
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }
}
