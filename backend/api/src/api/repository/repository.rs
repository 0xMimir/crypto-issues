use error::{Error, Result};
use sea_orm::{
    prelude::Uuid,
    sea_query::{extension::postgres::PgExpr, Expr},
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use std::sync::Arc;
use store::{
    github_projects, github_repositories,
    issues::{self, Model as Issues},
    objects::RepositoryView,
};
use support::pagination::Pagination;

use super::{contract::DbRepositoryContract, data::GetIssuesParams};

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
    async fn get_issues_for_repository(
        &self,
        repository_id: Uuid,
        params: GetIssuesParams,
    ) -> Result<Pagination<Issues>> {
        let mut query = issues::Entity::find().filter(issues::Column::Repository.eq(repository_id));

        if let Some(closed) = params.closed {
            query = query.filter(issues::Column::Closed.eq(closed));
        }

        if let Some(title) = params.search {
            query = query.filter(
                issues::Column::Title
                    .into_expr()
                    .ilike(format!("%{}%", title)),
            );
        }

        let order_by = params.order_by.unwrap_or("created_at".to_owned());
        let order = params.order.unwrap_or(support::order::Order::Desc);
        let per_page = params.per_page.unwrap_or(50);
        let page = params.page.unwrap_or_default();

        let query = query
            .order_by(Expr::cust(order_by), order.into())
            .into_model::<Issues>()
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

    async fn get_repository(&self, id: Uuid) -> Result<RepositoryView> {
        github_repositories::Entity::find_by_id(id)
            .inner_join(github_projects::Entity)
            .select_only()
            .column(github_repositories::Column::Id)
            .column_as(github_repositories::Column::RepositoryName, "name")
            .column(github_repositories::Column::Language)
            .column(github_repositories::Column::ForksCount)
            .column_as(github_repositories::Column::StargazersCount, "stars_count")
            .column_as(github_repositories::Column::Project, "project_id")
            .column_as(github_projects::Column::Name, "project")
            .into_model::<RepositoryView>()
            .one(self.conn.as_ref())
            .await?
            .ok_or(Error::NotFoundWithCause(format!("Repository: {}", id)))
    }
}
