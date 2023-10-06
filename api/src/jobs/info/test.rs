use std::time::Duration;

use sea_orm::EntityTrait;
use store::{cryptocurrencies, github_projects};
use support::db_pool::create_db_pool;
use tokio::time::sleep;

#[adtest(cleanup = async cleanup_db)]
async fn test_info() {
    config::dotenv_init();
    let sea_pool = create_db_pool().await;
    let job = super::create_info(sea_pool);

    let handle = tokio::spawn(async move {
        job.preform_init().await.expect("Error running init");
    });

    sleep(Duration::from_secs(30)).await;

    handle.abort();
}

async fn cleanup_db() {
    let sea_pool = create_db_pool().await;

    let _ = cryptocurrencies::Entity::delete_many()
        .exec(sea_pool.as_ref())
        .await;

    let _ = github_projects::Entity::delete_many()
        .exec(sea_pool.as_ref())
        .await;
}
