use error::{Error, Result};
use sdks::github::data::{GithubIssue, State};
use sea_orm::{
    prelude::Uuid,
    sea_query::OnConflict,
    ActiveValue::{NotSet, Set},
    DatabaseConnection, EntityTrait, TransactionTrait,
};
use std::sync::Arc;

use super::super::contract::DbServiceContract;
use store::{
    issue_labels,
    issues::{ActiveModel, Column, Entity},
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
    async fn create_issues(&self, repository_id: Uuid, issue: GithubIssue) -> Result<()> {
        self.conn
            .as_ref()
            .transaction::<_, (), sea_orm::error::DbErr>(|txn| {
                Box::pin(async move {
                    let model = ActiveModel {
                        repository: Set(repository_id),
                        issue: Set(issue.number),
                        title: Set(issue.title),
                        description: Set(issue.description),
                        created_at: Set(issue.created_at),
                        closed: Set(issue.state == State::Closed),
                        id: Default::default(),
                    };

                    let mut on_conflict = OnConflict::columns([Column::Repository, Column::Issue]);
                    on_conflict.update_columns([
                        Column::Title,
                        Column::Description,
                        Column::Closed,
                    ]);

                    let issue_id = Entity::insert(model)
                        .on_conflict(on_conflict)
                        .exec_with_returning(txn)
                        .await?
                        .id;

                    let labels = issue
                        .labels
                        .into_iter()
                        .map(|label| issue_labels::ActiveModel {
                            id: NotSet,
                            issue_id: Set(issue_id),
                            name: Set(label.name),
                        })
                        .collect::<Vec<_>>();

                    issue_labels::Entity::insert_many(labels).exec(txn).await?;

                    Ok(())
                })
            })
            .await
            .map_err(|e| Error::InternalServer(e.to_string()))?;

        Ok(())
    }
}
