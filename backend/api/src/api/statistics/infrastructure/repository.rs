use std::sync::Arc;

use crate::api::statistics::contract::DbRepositoryContract;
use error::{Error, Result};
use sea_orm::{
    sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder,
    QuerySelect,
};
use store::{github_repositories, objects::LanguageCount};

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
    async fn get_language_counts(&self) -> Result<Vec<LanguageCount>> {
        github_repositories::Entity::find()
            .select_only()
            .column(github_repositories::Column::Language)
            .column_as(Expr::col(github_repositories::Column::Id).count(), "count")
            .filter(github_repositories::Column::Language.is_not_null())
            .group_by(github_repositories::Column::Language)
            .order_by(
                Expr::col(github_repositories::Column::Id).count(),
                Order::Desc,
            )
            .into_model()
            .all(self.conn.as_ref())
            .await
            .map_err(Error::from)
    }
}
