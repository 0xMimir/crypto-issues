use error::Result;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use std::sync::Arc;

use super::super::contract::DbRepositoryContract;
use store::cryptocurrencies::{Column, Entity};

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
    async fn get_projects(&self) -> Result<Vec<String>> {
        let projects = Entity::find()
            .filter(Column::Github.is_not_null())
            .select_only()
            .column(Column::Github)
            .into_tuple()
            .all(self.conn.as_ref())
            .await?;

        Ok(projects)
    }
}
