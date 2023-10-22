use api::{create_api, setup_jobs};
use config::dotenv_init;
use sea_orm::Database;
use std::sync::Arc;
use support::logger::setup_logger;

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() {
    setup_logger();
    dotenv_init();

    let db_url = config::get("DATABASE_URL").unwrap();
    let pool = Database::connect(db_url).await.unwrap();
    let sea_pool = Arc::new(pool);

    if config::get_default("RUN_JOBS", "true") == "true" {
        let _handles = setup_jobs(sea_pool.clone());
    }

    create_api(sea_pool, 1111).await.expect("Error starting server");
}
