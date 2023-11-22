use error::Result;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::Arc;

use super::super::contract::DbServiceContract;
use store::github_projects;

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
    async fn update_project(&self, project: github_projects::ActiveModel) -> Result<()> {
        github_projects::Entity::update(project)
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }
}
