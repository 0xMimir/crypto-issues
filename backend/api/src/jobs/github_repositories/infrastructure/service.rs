use error::Result;
use sdks::github::data::GithubRepository;
use sea_orm::{
    prelude::Uuid, sea_query::OnConflict, ActiveValue::Set, DatabaseConnection, EntityTrait,
};
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
    async fn create_repository(
        &self,
        project: Uuid,
        repositories: Vec<GithubRepository>,
    ) -> Result<()> {
        let models = repositories
            .into_iter()
            .map(|repository| ActiveModel {
                id: Default::default(),
                project: Set(project.to_owned()),
                repository_name: Set(repository.name),
                language: Set(repository.language),
                stargazers_count: Set(repository.stargazers_count),
                forks_count: Set(repository.forks_count),
            })
            .collect::<Vec<_>>();

        let mut on_conflict = OnConflict::columns([Column::Project, Column::RepositoryName]);

        on_conflict.update_columns([
            Column::ForksCount,
            Column::StargazersCount,
            Column::Language,
        ]);

        Entity::insert_many(models)
            .on_conflict(on_conflict)
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }
}
