use cronus::Cronus;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
/// Public because of init
pub mod github_repositories;
pub mod info;

mod github_issues;
mod github_project_info;
mod github_topics;
mod github_topics_scraper;

pub fn setup(sea_pool: Arc<DatabaseConnection>) -> Cronus {
    let cron = Cronus::new();

    info::setup(&cron, sea_pool.clone());
    github_repositories::setup(&cron, sea_pool.clone());
    github_project_info::setup(&cron, sea_pool.clone());
    github_topics::setup(&cron, sea_pool.clone());
    github_topics_scraper::setup(&cron, sea_pool.clone());
    github_issues::setup(&cron, sea_pool);

    cron
}
