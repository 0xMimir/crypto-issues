mod contract;
mod domain;
pub mod infrastructure;
use std::sync::Arc;

pub use domain::Info;
use infrastructure::{PgRepository, PgService};
use sdks::coingecko::CoinGecko;
use sea_orm::DatabaseConnection;

#[cfg(test)]
mod test;

pub fn setup(sea_pool: Arc<DatabaseConnection>) -> tokio::task::JoinHandle<()> {
    let cron = create_info(sea_pool);
    cron.spawn_cron()
}

fn create_info(sea_pool: Arc<DatabaseConnection>) -> Info<PgRepository, PgService, CoinGecko> {
    let repository = PgRepository::new(sea_pool.clone());
    let service = PgService::new(sea_pool);
    let coingecko = CoinGecko::default();

    Info::new(repository, service, coingecko)
}
