use error::Result;
use sea_orm::{
    prelude::Uuid, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
};
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
    async fn get_assets_without_info(&self) -> Result<Vec<(Uuid, String)>> {
        let query = Entity::find()
            .filter(Column::Github.is_null())
            .filter(Column::Gitlab.is_null())
            .filter(Column::Description.is_null())
            .select_only()
            .columns([Column::Id, Column::CoingeckoId])
            .into_tuple()
            .all(self.conn.as_ref())
            .await?;

        // println!("{}", query.to_string());

        Ok(query)
    }
}
