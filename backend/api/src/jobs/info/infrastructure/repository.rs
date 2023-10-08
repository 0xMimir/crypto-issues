use error::Result;
use sea_orm::{
    prelude::Uuid, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
};
use std::{collections::HashMap, sync::Arc};

use super::super::contract::DbRepositoryContract;
use store::{
    cryptocurrencies::{Column, Entity},
    github_projects::{Column as GithubColumn, Entity as GithubEntity},
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

        Ok(query)
    }

    async fn get_projects(&self) -> Result<HashMap<String, Uuid>> {
        let projects = GithubEntity::find()
            .select_only()
            .columns([GithubColumn::Name, GithubColumn::Id])
            .into_tuple()
            .all(self.conn.as_ref())
            .await?
            .into_iter()
            .collect();

        Ok(projects)
    }
}
