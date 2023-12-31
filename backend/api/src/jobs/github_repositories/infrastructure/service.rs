use error::Result;
use sdks::github::data::GithubRepository;
use sea_orm::{
    prelude::Uuid, sea_query::OnConflict, ActiveValue::Set, DatabaseConnection, EntityTrait,
};
use std::sync::Arc;

use super::super::contract::DbServiceContract;
use store::{
    github_projects,
    github_repositories::{ActiveModel, Column, Entity},
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
                created_at: Set(repository.created_at),
                updated_at: Set(repository.updated_at),
                archived: Set(repository.archived),
                fork: Set(repository.fork),
            })
            .collect::<Vec<_>>();

        let mut on_conflict = OnConflict::columns([Column::Project, Column::RepositoryName]);

        on_conflict.update_columns([
            Column::StargazersCount,
            Column::ForksCount,
            Column::Language,
            Column::UpdatedAt,
            Column::Archived,
            Column::Fork,
        ]);

        Entity::insert_many(models)
            .on_conflict(on_conflict)
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }

    async fn delete_project(&self, id: Uuid) -> Result<()> {
        github_projects::Entity::delete_by_id(id)
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }
}
