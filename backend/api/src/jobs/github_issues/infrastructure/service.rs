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
                issue: Set(issue.number),
                title: Set(issue.title),
                description: Set(issue.description),
                created_at: Set(issue.created_at),
                closed: Set(issue.state == State::Closed),
                id: Default::default(),
            })
            .collect::<Vec<_>>();

        let mut on_conflict = OnConflict::columns([Column::Repository, Column::Issue]);
        on_conflict.update_columns([Column::Title, Column::Description, Column::Closed]);

        Entity::insert_many(models)
            .on_conflict(on_conflict)
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }
}
