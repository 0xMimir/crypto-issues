mod contract;
mod domain;
pub mod infrastructure;
#[cfg(test)]
mod test;

pub use domain::GithubRepositoriesCron;
use infrastructure::{PgRepository, PgService};
use sdks::github::Github;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

///
/// Create and spawn github repositories job
///
pub fn setup(sea_pool: Arc<DatabaseConnection>) -> tokio::task::JoinHandle<()> {
    let cron = create_gr(sea_pool);
    cron.spawn_cron()
}

///
/// Create GithubRepositoriesCron with default implementations
/// 
fn create_gr(
    sea_pool: Arc<DatabaseConnection>,
) -> GithubRepositoriesCron<PgRepository, PgService, Github> {
    let repository = PgRepository::new(sea_pool.clone());
    let service = PgService::new(sea_pool);
    let github_api_key = config::get("GITHUB_KEY").ok();

    let github = match github_api_key {
        Some(api_key) => Github::new_with_auth(api_key),
        None => Github::default(),
    };

    GithubRepositoriesCron::new(repository, service, github)
}
