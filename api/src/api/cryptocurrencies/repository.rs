use error::{Error, Result};
use sea_orm::{
    sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter,
    QuerySelect, RelationTrait, SelectColumns,
};
use std::sync::Arc;
use store::{
    cryptocurrencies, github_projects, github_repositories, issues, objects::CryptoCurrencyView,
};

use super::contract::DbRepositoryContract;

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
    async fn get_cryptocurrencies(&self) -> Result<Vec<CryptoCurrencyView>> {
        github_projects::Entity::find()
            .inner_join(cryptocurrencies::Entity)
            .left_join(github_repositories::Entity)
            .join_rev(
                JoinType::LeftJoin,
                issues::Relation::GithubRepositories.def(),
            )
            .filter(github_repositories::Column::RepositoryName.is_not_null())
            .group_by(cryptocurrencies::Column::Id)
            .group_by(cryptocurrencies::Column::Name)
            .group_by(cryptocurrencies::Column::CoingeckoId)
            .group_by(cryptocurrencies::Column::Github)
            .group_by(github_projects::Column::Name)
            .select_only()
            .columns([
                cryptocurrencies::Column::Id,
                cryptocurrencies::Column::Name,
                cryptocurrencies::Column::CoingeckoId,
            ])
            .select_column_as(cryptocurrencies::Column::Github, "github_id")
            .column_as(github_projects::Column::Name, "github")
            .expr(Expr::cust_with_expr(
                "array_agg(distinct $1) as repositories",
                github_repositories::Column::RepositoryName.into_expr(),
            ))
            .column_as(issues::Column::Id.count(), "issues")
            .into_model::<CryptoCurrencyView>()
            .all(self.conn.as_ref())
            .await
            .map_err(Error::from)
    }
}