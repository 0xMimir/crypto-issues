use error::Result;
use sea_orm::{
    sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait, IntoSimpleExpr, JoinType,
    PaginatorTrait, QueryOrder, QuerySelect, RelationTrait,
};
use std::sync::Arc;
use store::{github_projects, github_repositories, issues, objects::SearchGithubProject};
use support::pagination::Pagination;

use super::super::{contract::DbRepositoryContract, data::SearchGithubProjectParams};

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
    async fn search(
        &self,
        params: SearchGithubProjectParams,
    ) -> Result<Pagination<SearchGithubProject>> {
        let query = github_projects::Entity::find()
            .select_only()
            .columns([github_projects::Column::Id, github_projects::Column::Name])
            .inner_join(github_repositories::Entity)
            .column_as(github_repositories::Column::Id.count(), "repositories")
            .column_as(
                Expr::cust_with_expr(
                    r#"coalesce(array_agg(distinct $1) filter(where $1 notnull), '{}')"#,
                    github_repositories::Column::Language.into_simple_expr(),
                ),
                "languages_used",
            )
            .join_rev(
                JoinType::LeftJoin,
                issues::Relation::GithubRepositories.def(),
            )
            .column_as(issues::Column::Id.count(), "issues");

        let order_by = params.order_by.unwrap_or("issues".to_owned());
        let order = params.order.unwrap_or(support::order::Order::Desc);
        let per_page = params.per_page.unwrap_or(50);
        let page = params.page.unwrap_or_default();

        let query = query
            .order_by(Expr::cust(order_by), order.into())
            .into_model::<SearchGithubProject>()
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
}
