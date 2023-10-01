use std::{sync::Arc, time::Duration};

use api::jobs::info::{
    infrastructure::{PgRepository, PgService},
    Info,
};
use config::dotenv_init;
use sdks::coingecko::CoinGecko;
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
    let conn = Arc::new(pool);

    let repository = PgRepository::new(conn.clone());
    let service = PgService::new(conn);
    let coingecko = CoinGecko::default();

    let init = Info::new(repository, service, coingecko);

    let _handle = init.spawn_cron();

    let mut period = interval(Duration::from_millis(100));
    loop {
        period.tick().await;
    }
}
