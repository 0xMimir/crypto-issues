use super::{
    contract::{CryptocurrenciesContract, DbRepositoryContract},
    data::GetCryptoCurrenciesQuery,
};
use error::Result;
use sea_orm::prelude::Uuid;
use store::objects::{CryptoCurrencyView, CryptoCurrencyWithRepositories};
use support::pagination::Pagination;

pub struct Cryptocurrencies<A: DbRepositoryContract> {
    pub(super) repository: A,
}

#[async_trait]
impl<A: DbRepositoryContract + Send + Sync> CryptocurrenciesContract for Cryptocurrencies<A> {
    async fn get_cryptocurrencies(
        &self,
        query: GetCryptoCurrenciesQuery,
    ) -> Result<Pagination<CryptoCurrencyView>> {
        self.repository.get_cryptocurrencies(query).await
    }
    async fn get_cryptocurrency(&self, id: Uuid) -> Result<CryptoCurrencyWithRepositories> {
        self.repository.get_cryptocurrency(id).await
    }
}
