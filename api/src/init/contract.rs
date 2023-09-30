use apis::coingecko::CryptoInfo;
use error::Result;
use sea_orm::prelude::Uuid;

pub trait DbRepositoryContract {
    ///
    /// Get items from `cryptocurrencies` that don't have neither github, nor gitlab, nor description
    ///
    async fn get_assets_without_info(&self) -> Result<Vec<(Uuid, String)>>;
}

pub trait DbServiceContract {
    ///
    /// Insert into `cryptocurrencies` table
    ///
    async fn insert_crypto(&self, name: String, coingecko_id: String) -> Result<()>;

    ///
    /// Update cryptocurrencies with id of `id` and set not null values from info
    ///
    async fn update_info(&self, id: Uuid, info: CryptoInfo) -> Result<()>;
}
