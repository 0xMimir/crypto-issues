use error::Result;
use sea_orm::{
    sea_query::{extension::postgres::PgExpr, Alias, Expr, PostgresQueryBuilder, SeaRc},
    ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, IntoSimpleExpr, JoinType,
    PaginatorTrait, QueryOrder, QuerySelect, QueryTrait, Statement,
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
        let sub_select = github_repositories::Entity::find()
            .left_join(issues::Entity)
            .column_as(issues::Column::Id.count(), "issues")
            .group_by(github_repositories::Column::Id)
            .into_query();

        let mut query = github_projects::Entity::find()
            .column_as(
                Expr::cust_with_expr(
                    r#"count(distinct $1)"#,
                    github_repositories::Column::Id.into_simple_expr(),
                ),
                "repositories",
            )
            .column_as(
                Expr::cust_with_expr(
                    r#"coalesce(array_agg(distinct $1) filter(where $1 notnull), '{}')"#,
                    github_repositories::Column::Language.into_simple_expr(),
                ),
                "languages_used",
            )
            .column_as(
                Expr::cust(r#"sum("github_repositories"."issues")::bigint"#),
                "issues",
            )
            .column_as(
                Expr::cust(r#"sum("github_repositories"."stargazers_count")::bigint"#),
                "stargazers_count",
            )
            .group_by(github_projects::Column::Id);

        if let Some(languages) = params.languages_used {
            if !languages.is_empty() {
                query = query.having(
                    Expr::cust_with_expr(
                        r#"coalesce(array_agg(distinct $1) filter(where $1 notnull), '{}')::text[]"#,
                        github_repositories::Column::Language.into_simple_expr(),
                    )
                    .contains(languages),
                );
            }
        }

        let order_by = params.order_by.unwrap_or("issues".to_owned());
        let order = params.order.unwrap_or(support::order::Order::Desc);
        let per_page = params.per_page.unwrap_or(50);
        let page = params.page.unwrap_or_default();

        let query = query
            .order_by(Expr::cust(order_by), order.into())
            .into_query()
            .join_subquery(
                JoinType::InnerJoin,
                sub_select,
                SeaRc::new(Alias::new("github_repositories")),
                github_repositories::Column::Project
                    .into_expr()
                    .eq(github_projects::Column::Id.into_expr()),
            )
            .to_owned();

        let (sql, values) = query.build(PostgresQueryBuilder);

        let query = github_projects::Entity::find().from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            values,
        ));

        let query = query
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
