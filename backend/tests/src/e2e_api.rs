use sea_orm::DatabaseConnection;
use std::{future::Future, sync::Arc, time::Duration};
use tokio::time::sleep;

pub async fn run_e2e_api_tests<F, T>(sea_pool: Arc<DatabaseConnection>, test_function: F)
where
    F: Fn() -> T,
    T: Future<Output = ()>,
{
    let routes = api::create_api(sea_pool);

    let handle = tokio::spawn(async move { routes.await });

    sleep(Duration::from_secs(1)).await;

    test_function().await;

    handle.abort();
}
