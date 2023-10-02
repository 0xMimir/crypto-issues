use api::{create_api, setup_jobs};
use config::dotenv_init;
use sea_orm::Database;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn) // Set to warn so it doesn't print sea orm queries
        .init();

    dotenv_init();
    let db_url = config::get("DATABASE_URL").unwrap();
    let pool = Database::connect(db_url).await.unwrap();
    let sea_pool = Arc::new(pool);

    let _handles = setup_jobs(sea_pool.clone());

    create_api(sea_pool).await.expect("Error starting server");
}
