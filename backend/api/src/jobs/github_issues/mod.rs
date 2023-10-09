mod contract;
mod domain;
mod infrastructure;

use std::sync::Arc;

use domain::GithubIssueCron;
use infrastructure::{PgRepository, PgService};
use sdks::github::Github;
use sea_orm::DatabaseConnection;

#[cfg(test)]
mod test;

pub fn setup(sea_pool: Arc<DatabaseConnection>) -> tokio::task::JoinHandle<()> {
    let cron = setup_github_issues(sea_pool);
    cron.spawn_cron()
}

fn setup_github_issues(
    sea_pool: Arc<DatabaseConnection>,
) -> GithubIssueCron<PgRepository, PgService, Github> {
    let repository = PgRepository::new(sea_pool.clone());
    let service = PgService::new(sea_pool);

    let github_api_key = config::get("GITHUB_KEY").ok();

    let github = match github_api_key {
        Some(api_key) => Github::new_with_auth(api_key),
        None => Github::default(),
    };

    GithubIssueCron::new(repository, service, github)
}