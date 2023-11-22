use error::Result;
use sea_orm::{DatabaseConnection, EntityTrait, Order, QueryOrder};
use std::sync::Arc;

use super::super::contract::DbRepositoryContract;
use store::github_projects::{Column, Entity, Model};

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
        let projects = Entity::find()
            .order_by(Column::Name, Order::Asc)
            .all(self.conn.as_ref())
            .await?;

        Ok(projects)
    }
}
