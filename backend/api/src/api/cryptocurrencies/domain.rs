use super::{
    contract::{CryptocurrenciesContract, DbRepositoryContract},
    data::GetCryptoCurrenciesQuery,
};
use error::Result;
use sea_orm::prelude::Uuid;
use store::{
    issues::Model as Issue,
    objects::{CryptoCurrencyView, CryptoCurrencyWithRepositories},
};
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

    async fn get_issues_for_repository(&self, repository_id: Uuid) -> Result<Vec<Issue>> {
        self.repository
            .get_issues_for_repository(repository_id)
            .await
    }
}
