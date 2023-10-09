use error::Result;
use sea_orm::prelude::Uuid;
use store::objects::{CryptoCurrencyView, CryptoCurrencyWithRepositories};
use support::pagination::Pagination;

use super::data::GetCryptoCurrenciesQuery;

#[async_trait]
pub trait DbRepositoryContract {
    async fn get_cryptocurrency(&self, id: Uuid) -> Result<CryptoCurrencyWithRepositories>;
    async fn get_cryptocurrencies(
        &self,
        query: GetCryptoCurrenciesQuery,
    ) -> Result<Pagination<CryptoCurrencyView>>;
}

#[async_trait]
pub trait CryptocurrenciesContract {
    async fn get_cryptocurrency(&self, id: Uuid) -> Result<CryptoCurrencyWithRepositories>;
    async fn get_cryptocurrencies(
        &self,
        query: GetCryptoCurrenciesQuery,
    ) -> Result<Pagination<CryptoCurrencyView>>;
}
