use sea_orm::Database;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

#[tokio::test]
async fn e2e_api() {
    let db_url = config::get("DATABASE_URL").unwrap();
    let pool = Database::connect(db_url).await.unwrap();
    let sea_pool = Arc::new(pool);

    let routes = api::create_api(sea_pool);

    let handle = tokio::spawn(async move { routes.await });

    sleep(Duration::from_secs(1)).await;

    handle.abort();
}
