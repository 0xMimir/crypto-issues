use error::Result;
use reqwest::get;
use sea_orm::Database;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

#[tokio::test]
////
/// This test doesn't take variables from .env, it is used to know which values MUST
/// be in env for api to start, not to function
///
async fn test_route_setup() -> Result<()> {
    let db_url = config::get("DATABASE_URL").unwrap();
    let pool = Database::connect(db_url).await.unwrap();
    let sea_pool = Arc::new(pool);

    let routes = api::create_api(sea_pool);

    let handle = tokio::spawn(async move { routes.await });

    sleep(Duration::from_secs(1)).await;

    let response = get("http://127.0.0.1:1111/hello-there")
        .await?
        .text()
        .await?;

    assert_eq!(response, "General Kenobi");
    handle.abort();

    Ok(())
}
