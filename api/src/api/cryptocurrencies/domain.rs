use super::contract::{CryptocurrenciesContract, DbRepositoryContract};
use error::Result;
use store::objects::CryptoCurrencyView;

pub struct Cryptocurrencies<A: DbRepositoryContract> {
    pub(super) repository: A,
}

#[async_trait]
impl<A: DbRepositoryContract + Send + Sync> CryptocurrenciesContract for Cryptocurrencies<A> {
    async fn get_cryptocurrencies(&self) -> Result<Vec<CryptoCurrencyView>> {
        self.repository.get_cryptocurrencies().await
    }
}
