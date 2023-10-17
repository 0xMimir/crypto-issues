use error::{Error, Result};
use sea_orm::{
    prelude::Uuid,
    sea_query::{extension::postgres::PgExpr, Expr},
    ColumnTrait, DatabaseConnection, EntityTrait, IntoSimpleExpr, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use std::sync::Arc;
use store::{
    github_projects, github_repositories, issues,
    objects::{GithubIssue, RepositoryView, SearchRepository},
};
use support::pagination::Pagination;

use super::{
    contract::DbRepositoryContract,
    data::{GetIssuesParams, SearchRepositoryParams},
};

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
    ) -> Result<Pagination<GithubIssue>> {
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
            .into_model::<GithubIssue>()
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

    async fn search_repositories(
        &self,
        params: SearchRepositoryParams,
    ) -> Result<Pagination<SearchRepository>> {
        let mut query = github_repositories::Entity::find()
            .inner_join(github_projects::Entity)
            .select_only()
            .columns([
                github_repositories::Column::Id,
                github_repositories::Column::RepositoryName,
                github_repositories::Column::Language,
                github_repositories::Column::StargazersCount,
                github_repositories::Column::ForksCount,
                github_repositories::Column::CreatedAt,
                github_repositories::Column::UpdatedAt,
                github_repositories::Column::Archived,
            ])
            .column_as(github_repositories::Column::Project, "project_id")
            .column_as(github_projects::Column::Name, "project")
            .column_as(
                Expr::cust_with_exprs(
                    "concat('https://github.com/', $1, '/', $2)",
                    [
                        github_projects::Column::Name.into_simple_expr(),
                        github_repositories::Column::RepositoryName.into_simple_expr(),
                    ],
                ),
                "url",
            );

        if let Some(language) = params.language {
            query = query.filter(github_repositories::Column::Language.eq(language))
        }

        if let Some(repository) = params.repository {
            query = query.filter(
                github_repositories::Column::RepositoryName
                    .into_expr()
                    .ilike(format!("%{}%", repository)),
            );
        }

        if let Some(archived) = params.archived {
            query = query.filter(github_repositories::Column::Archived.eq(archived));
        }

        if let Some(project_id) = params.project_id {
            query = query.filter(github_projects::Column::Id.eq(project_id));
        }

        if let Some(project) = params.project {
            query = query.filter(github_projects::Column::Name.eq(project));
        }

        if let Some(fork) = params.fork {
            query = query.filter(github_repositories::Column::Fork.eq(fork));
        }

        let order_by = params.order_by.unwrap_or("updated_at".to_owned());
        let order = params.order.unwrap_or(support::order::Order::Desc);
        let per_page = params.per_page.unwrap_or(50);
        let page = params.page.unwrap_or_default();

        let query = query
            .order_by(Expr::cust(order_by), order.into())
            .into_model::<SearchRepository>()
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
