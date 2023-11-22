use error::Result;
use reqwest::get;
use std::time::Duration;
use support::db_pool::create_db_pool;
use tokio::time::sleep;

#[tokio::test]
////
/// This test doesn't take variables from .env, it is used to know which values MUST
/// be in env for api to start, not to function
///
async fn test_route_setup() -> Result<()> {
    let sea_pool = create_db_pool().await;

    let routes = api::create_api(sea_pool, 1111);

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

// #[tokio::test]
// async fn test_job_setup() {
//     let db_pool = create_db_pool().await;
//     let jobs = api::setup_jobs(db_pool);
//     sleep(Duration::from_secs(5)).await;
//     for job in jobs {
//         assert!(!job.is_finished());
//         job.abort();
//     }
// }
