use error::Result;
use sea_orm::prelude::Uuid;
use store::{
    issues::Model as Issues,
    objects::{CryptoCurrencyView, CryptoCurrencyWithRepositories},
};

use super::data::GetCryptoCurrenciesQuery;

#[async_trait]
pub trait DbRepositoryContract {
    async fn get_issues_for_repository(&self, repository_id: Uuid) -> Result<Vec<Issues>>;
    async fn get_cryptocurrency(&self, id: Uuid) -> Result<CryptoCurrencyWithRepositories>;
    async fn get_cryptocurrencies(&self, query: GetCryptoCurrenciesQuery) -> Result<Vec<CryptoCurrencyView>>;
}

#[async_trait]
pub trait CryptocurrenciesContract {
    async fn get_issues_for_repository(&self, repository_id: Uuid) -> Result<Vec<Issues>>;
    async fn get_cryptocurrency(&self, id: Uuid) -> Result<CryptoCurrencyWithRepositories>;
    async fn get_cryptocurrencies(&self, query: GetCryptoCurrenciesQuery) -> Result<Vec<CryptoCurrencyView>>;
}
