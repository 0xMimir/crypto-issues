mod contract;
mod domain;
pub mod infrastructure;


use cronus::Cronus;
pub use domain::GithubProjectInfo;
use infrastructure::{PgRepository, PgService};
use sdks::github::Github;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

///
/// Create and spawn github repositories job
///
pub fn setup(cron: &Cronus, sea_pool: Arc<DatabaseConnection>) {
    let job = create_gr(sea_pool);
    cron.add(job).expect("Error adding job");
}

///
/// Create GithubRepositoriesCron with default implementations
///
fn create_gr(
    sea_pool: Arc<DatabaseConnection>,
) -> GithubProjectInfo<PgRepository, PgService, Github> {
    let repository = PgRepository::new(sea_pool.clone());
    let service = PgService::new(sea_pool);
    let github_api_key = config::get("GITHUB_KEY").ok();

    let github = match github_api_key {
        Some(api_key) => Github::new_with_auth(api_key),
        None => Github::default(),
    };

    GithubProjectInfo::new(repository, service, github)
}
