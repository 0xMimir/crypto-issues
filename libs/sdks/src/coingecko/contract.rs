use error::Result;

use super::data::{CryptoInfo, SimpleCoin};

#[async_trait]
pub trait CoinGeckoContract {
    ///
    /// Return all assets for coingecko
    ///
    async fn list_cryptocurrencies(&self) -> Result<Vec<SimpleCoin>>;

    ///
    /// Return github info for coingecko id
    ///
    async fn get_info(&self, id: &str) -> Result<CryptoInfo>;
}
