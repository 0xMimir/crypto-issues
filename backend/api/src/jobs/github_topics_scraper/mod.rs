use std::sync::Arc;

use cronus::Cronus;
use sdks::github::Github;
use sea_orm::DatabaseConnection;

use self::{
    domain::GithubTopicsScraper,
    infrastructure::{PgRepository, PgService},
};

mod contract;
mod domain;
mod infrastructure;

pub fn setup(cron: &Cronus, sea_pool: Arc<DatabaseConnection>) {
    let job = create_gr(sea_pool);
    cron.add(job).expect("Error adding job");
}

fn create_gr(
    sea_pool: Arc<DatabaseConnection>,
) -> GithubTopicsScraper<PgRepository, PgService, Github> {
    let repository = PgRepository::new(sea_pool.clone());
    let service = PgService::new(sea_pool);
    let github_api_key = config::get("GITHUB_KEY").ok();

    let github = match github_api_key {
        Some(api_key) => Github::new_with_auth(api_key),
        None => Github::default(),
    };

    GithubTopicsScraper::new(repository, service, github)
}
