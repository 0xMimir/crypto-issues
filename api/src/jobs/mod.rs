use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub mod github_repositories;
pub mod info;

pub fn setup(sea_pool: Arc<DatabaseConnection>) -> [JoinHandle<()>; 2] {
    [
        info::setup(sea_pool.clone()),
        github_repositories::setup(sea_pool),
    ]
}
