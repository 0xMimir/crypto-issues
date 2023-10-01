use std::{sync::Arc, time::Duration};

use api::jobs;
use config::dotenv_init;
use sea_orm::Database;
use tokio::time::interval;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    dotenv_init();
    let db_url = config::get("DATABASE_URL").unwrap();
    let pool = Database::connect(db_url).await.unwrap();
    let sea_pool = Arc::new(pool);

    let _handles = jobs::setup(sea_pool);

    let mut period = interval(Duration::from_millis(100));
    loop {
        period.tick().await;
    }
}
