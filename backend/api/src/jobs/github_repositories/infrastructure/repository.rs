use error::Result;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::Arc;

use super::super::contract::DbRepositoryContract;
use store::github_projects::{Entity, Model};

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
    async fn get_projects(&self) -> Result<Vec<Model>> {
        let projects = Entity::find().all(self.conn.as_ref()).await?;
        Ok(projects)
    }
}
