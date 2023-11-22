mod contract;
mod domain;
pub mod infrastructure;
use std::sync::Arc;

use cronus::Cronus;
pub use domain::Info;
use infrastructure::{PgRepository, PgService};
use sdks::coingecko::CoinGecko;
use sea_orm::DatabaseConnection;

#[cfg(test)]
mod test;

pub fn setup(cron: &Cronus, sea_pool: Arc<DatabaseConnection>) {
    let job = create_info(sea_pool);
    cron.add(job).expect("Error adding job");
}

fn create_info(sea_pool: Arc<DatabaseConnection>) -> Info<PgRepository, PgService, CoinGecko> {
    let repository = PgRepository::new(sea_pool.clone());
    let service = PgService::new(sea_pool);
    let coingecko = CoinGecko::default();

    Info::new(repository, service, coingecko)
}
