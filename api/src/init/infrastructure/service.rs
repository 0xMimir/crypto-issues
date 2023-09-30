use apis::coingecko::CryptoInfo;
use error::Result;
use sea_orm::{
    prelude::Uuid, sea_query::OnConflict, ActiveModelTrait, ActiveValue::Set, DatabaseConnection,
    EntityTrait, Unchanged,
};
use std::sync::Arc;

use super::super::contract::DbServiceContract;
use store::cryptocurrencies::{ActiveModel, Entity};

pub struct PgService {
    conn: Arc<DatabaseConnection>,
}

impl PgService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

impl DbServiceContract for PgService {
    async fn insert_crypto(&self, name: String, coingecko_id: String) -> Result<()> {
        let value = ActiveModel {
            coingecko_id: Set(coingecko_id),
            name: Set(name),
            ..Default::default()
        };

        Entity::insert(value)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .on_empty_do_nothing()
            .exec_without_returning(self.conn.as_ref())
            .await?;

        Ok(())
    }

    async fn update_info(&self, id: Uuid, info: CryptoInfo) -> Result<()> {
        let mut model = ActiveModel {
            id: Unchanged(id),
            ..Default::default()
        };

        if info.all_none() {
            model.description = Set(Some("No info".to_owned()));
        }

        if let Some(github) = info.github {
            model.github = Set(Some(github));
        }

        if let Some(gitlab) = info.gitlab {
            model.gitlab = Set(Some(gitlab));
        }

        if let Some(description) = info.description {
            model.description = Set(Some(description));
        }

        model.update(self.conn.as_ref()).await?;

        Ok(())
    }
}
