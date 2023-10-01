use std::sync::Arc;

use api::jobs::info::{
    infrastructure::{PgRepository, PgService},
    Info,
};
use config::dotenv_init;
use sdks::coingecko::CoinGecko;
use sea_orm::Database;

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

    init.preform_init().await.unwrap();
}
