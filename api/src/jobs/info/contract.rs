use error::Result;
use sdks::coingecko::{CryptoInfo, SimpleCoin};
use sea_orm::prelude::Uuid;

#[async_trait]
pub trait DbRepositoryContract {
    ///
    /// Get items from `cryptocurrencies` that don't have neither github, nor gitlab, nor description
    ///
    async fn get_assets_without_info(&self) -> Result<Vec<(Uuid, String)>>;
}

#[async_trait]
pub trait DbServiceContract {
    ///
    /// Insert into `cryptocurrencies` table
    ///
    async fn insert_crypto(&self, cryptocurrencies: Vec<SimpleCoin>) -> Result<()>;

    ///
    /// Update cryptocurrencies with id of `id` and set not null values from info
    ///
    async fn update_info(&self, id: Uuid, info: CryptoInfo, github: Option<Uuid>) -> Result<()>;

    ///
    /// Create entry in `github_projects` table
    ///
    async fn create_github(&self, project: String) -> Result<Uuid>;
}
