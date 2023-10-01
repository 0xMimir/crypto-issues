use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::task::JoinHandle;

/// Public because of init
pub mod github_repositories;
pub mod info;

mod github_issues;

pub fn setup(sea_pool: Arc<DatabaseConnection>) -> [JoinHandle<()>; 3] {
    [
        info::setup(sea_pool.clone()),
        github_repositories::setup(sea_pool.clone()),
        github_issues::setup(sea_pool)
    ]
}
