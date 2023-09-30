use error::Result;

use super::data::SimpleCoin;

pub trait CoinGeckoContract {
    ///
    /// Return all assets for coingecko
    /// 
    async fn list_cryptocurrencies(&self) -> Result<Vec<SimpleCoin>>;

    /// 
    /// Return github info for coingecko id
    /// 
    async fn get_github(&self, id: &str) -> Result<String>;
}
