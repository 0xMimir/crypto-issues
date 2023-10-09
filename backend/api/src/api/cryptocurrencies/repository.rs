use error::{Error, Result};
use sea_orm::{
    prelude::Uuid,
    sea_query::{extension::postgres::PgExpr, Expr},
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait, SelectColumns,
};
use std::sync::Arc;
use store::{
    cryptocurrencies, github_projects, github_repositories, issues,
    objects::{CryptoCurrencyView, CryptoCurrencyWithRepositories, Repository},
};
use support::pagination::Pagination;

use super::{contract::DbRepositoryContract, data::GetCryptoCurrenciesQuery};

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
    async fn get_cryptocurrencies(
        &self,
        params: GetCryptoCurrenciesQuery,
    ) -> Result<Pagination<CryptoCurrencyView>> {
        let mut query = github_projects::Entity::find()
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
            .column_as(issues::Column::Id.count(), "issues");

        if let Some(search) = params.search {
            query = query.filter(
                cryptocurrencies::Column::Name
                    .into_expr()
                    .ilike(format!("%{}%", search)),
            );
        }

        let order_by = params.order_by.unwrap_or("issues".to_owned());
        let order = params.order.unwrap_or(support::order::Order::Desc);
        let per_page = params.per_page.unwrap_or(50);
        let page = params.page.unwrap_or_default();

        let query = query
            .order_by(Expr::cust(order_by), order.into())
            .into_model::<CryptoCurrencyView>()
            .paginate(self.conn.as_ref(), per_page);

        let pagination = query.num_items_and_pages().await?;
        let data = query.fetch_page(page).await?;

        Ok(Pagination {
            page,
            per_page,
            order_by: vec![],
            data,
            total_items: pagination.number_of_items,
            last_page: pagination.number_of_pages,
        })
    }

    async fn get_cryptocurrency(&self, id: Uuid) -> Result<CryptoCurrencyWithRepositories> {
        let cryptocurrency = cryptocurrencies::Entity::find_by_id(id)
            .one(self.conn.as_ref())
            .await?
            .ok_or(Error::NotFoundWithCause(id.to_string()))?;

        let github_id = cryptocurrency
            .github
            .ok_or(Error::NotFoundWithCause(format!(
                "Github profile not found, id:{}",
                id
            )))?;

        let github = github_projects::Entity::find_by_id(github_id)
            .one(self.conn.as_ref())
            .await?
            .ok_or(Error::NotFoundWithCause(format!(
                "Github not found, id:{}",
                github_id.to_string()
            )))?;

        let repositories = github_repositories::Entity::find()
            .filter(github_repositories::Column::Project.eq(github.id))
            .select_only()
            .columns([
                github_repositories::Column::Id,
                github_repositories::Column::RepositoryName,
            ])
            .into_model::<Repository>()
            .all(self.conn.as_ref())
            .await?;

        let repository_ids = repositories
            .iter()
            .map(|repository| repository.id)
            .collect::<Vec<_>>();

        let issues = issues::Entity::find()
            .filter(issues::Column::Repository.is_in(repository_ids))
            .count(self.conn.as_ref())
            .await?;

        Ok(CryptoCurrencyWithRepositories {
            id: cryptocurrency.id,
            name: cryptocurrency.name,
            coingecko_id: cryptocurrency.coingecko_id,
            github_id,
            github: github.name,
            repositories,
            issues,
        })
    }
}
