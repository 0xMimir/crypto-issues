mod contract;
mod domain;
pub mod infrastructure;
use std::sync::Arc;

pub use domain::Info;
use infrastructure::{PgRepository, PgService};
use sdks::coingecko::CoinGecko;
use sea_orm::DatabaseConnection;

pub fn setup(sea_pool: Arc<DatabaseConnection>) -> tokio::task::JoinHandle<()> {
    let repository = PgRepository::new(sea_pool.clone());
    let service = PgService::new(sea_pool);
    let coingecko = CoinGecko::default();

    let cron = Info::new(repository, service, coingecko);
    cron.spawn_cron()
}
