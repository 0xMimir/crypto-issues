use error::Result;
use sdks::github::data::{GithubIssue, State};
use sea_orm::{
    prelude::Uuid, sea_query::OnConflict, ActiveValue::Set, DatabaseConnection, EntityTrait,
};
use std::sync::Arc;

use super::super::contract::DbServiceContract;
use store::issues::{ActiveModel, Column, Entity};

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
    async fn create_issues(&self, repository_id: Uuid, issues: Vec<GithubIssue>) -> Result<()> {
        let models = issues
            .into_iter()
            .map(|issue| ActiveModel {
                repository: Set(repository_id),
                issue: Set(issue.id),
                title: Set(issue.title),
                description: Set(Some(issue.description)),
                created_at: Set(issue.created_at),
                closed: Set(issue.state == State::Closed),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        Entity::insert_many(models)
            .on_conflict(
                OnConflict::columns([Column::Repository, Column::Issue])
                    .update_columns([Column::Title, Column::Description, Column::Closed])
                    .to_owned(),
            )
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }
}
