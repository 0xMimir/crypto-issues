use error::Result;
use sdks::coingecko::{CryptoInfo, SimpleCoin};
use sea_orm::{
    prelude::Uuid, sea_query::OnConflict, ActiveModelTrait, ActiveValue::Set, DatabaseConnection,
    EntityTrait, Unchanged,
};
use std::sync::Arc;

use super::super::contract::DbServiceContract;
use store::{
    cryptocurrencies::{ActiveModel, Entity, self},
    github_projects::{ActiveModel as CreateGithub, Entity as GithubEntity},
};

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
    async fn insert_crypto(&self, crypto: Vec<SimpleCoin>) -> Result<()> {
        let models = crypto
            .into_iter()
            .map(|simple_coin| ActiveModel {
                coingecko_id: Set(simple_coin.id),
                name: Set(simple_coin.name),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        Entity::insert_many(models)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .on_empty_do_nothing()
            .exec_without_returning(self.conn.as_ref())
            .await?;

        Ok(())
    }

    async fn update_info(&self, id: Uuid, info: CryptoInfo, github: Option<Uuid>) -> Result<()> {
        let mut model = ActiveModel {
            id: Unchanged(id),
            ..Default::default()
        };

        if info.all_none() {
            model.description = Set(Some("No info".to_owned()));
        }

        if let Some(github) = github {
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

    async fn create_github(&self, project: String) -> Result<Uuid> {
        let model = CreateGithub {
            name: Set(project),
            ..Default::default()
        };

        let model = GithubEntity::insert(model)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec_with_returning(self.conn.as_ref())
            .await?;

        Ok(model.id)
    }

    async fn delete_crypto(&self, id: Uuid) -> Result<()>{
        cryptocurrencies::Entity::delete_by_id(id).exec(self.conn.as_ref()).await?;
        Ok(())
    }
}
