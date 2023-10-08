use std::time::Duration;

use sea_orm::EntityTrait;
use store::{cryptocurrencies, github_projects};
use support::{count::count, db_pool::create_db_pool};
use tokio::time::sleep;

#[adtest(
    setup = async cleanup_db,
    cleanup = async cleanup_db
)]
#[serial_test::serial]
async fn test_info() {
    config::dotenv_init();
    let sea_pool = create_db_pool().await;
    let job = super::create_info(sea_pool.clone());

    let initial_count = count::<cryptocurrencies::Entity>(&sea_pool)
        .await
        .expect("Error getting count");

    let handle = tokio::spawn(async move {
        job.preform_init().await.expect("Error running init");
    });

    sleep(Duration::from_secs(30)).await;

    handle.abort();

    let count = count::<cryptocurrencies::Entity>(&sea_pool)
        .await
        .expect("Error getting count");

    assert!(count > initial_count);
}

async fn cleanup_db() {
    config::dotenv_init();
    let sea_pool = create_db_pool().await;

    let _ = cryptocurrencies::Entity::delete_many()
        .exec(sea_pool.as_ref())
        .await;

    let _ = github_projects::Entity::delete_many()
        .exec(sea_pool.as_ref())
        .await;
}
