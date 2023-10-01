mod contract;
mod domain;
pub mod infrastructure;

pub use domain::GithubRepositoriesCron;
use infrastructure::{PgRepository, PgService};
use sdks::github::Github;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub fn setup(sea_pool: Arc<DatabaseConnection>) -> tokio::task::JoinHandle<()> {
    let repository = PgRepository::new(sea_pool.clone());
    let service = PgService::new(sea_pool);
    let coingecko = Github::default();

    let cron = GithubRepositoriesCron::new(repository, service, coingecko);
    cron.spawn_cron()
}
