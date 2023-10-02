use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QuerySelect};
use std::sync::Arc;
use store::{cryptocurrencies, github_projects, github_repositories, issues};

pub struct PgRepository {
    conn: Arc<DatabaseConnection>,
}

impl PgRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

impl PgRepository {
    pub fn fetch(&self) {
        let query = github_projects::Entity::find()
            .inner_join(cryptocurrencies::Entity)
            .left_join(github_repositories::Entity)
            .filter(github_repositories::Column::RepositoryName.is_not_null())
            .group_by(cryptocurrencies::Column::Id)
            .group_by(cryptocurrencies::Column::Name)
            .group_by(cryptocurrencies::Column::CoingeckoId)
            .group_by(cryptocurrencies::Column::Github  )
            .group_by(github_projects::Column::Name) ;
    }
}
/*
select
	c.id,
	c."name",
	c.coingecko_id,
	c.github,
	gp."name",
	array_agg(gr.repository_name),
	count(i)
from
	cryptocurrencies c
inner join github_projects gp on
	gp.id = c.github
left join github_repositories gr on
	gr.project = gp.id
left join issues i on
	i.repository = gr.id
where gr.repository_name notnull
group by 
	c.id,
	c."name",
	c.coingecko_id,
	c.github,
	gp."name"
*/