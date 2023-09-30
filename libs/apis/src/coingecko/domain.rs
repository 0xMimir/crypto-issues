use super::{data::CryptoInfo, CoinGeckoContract, SimpleCoin};
use error::Result;
use reqwest::Client;

const COINS_LIST_URL: &str = "https://api.coingecko.com/api/v3/coins/list";

#[derive(Default)]
pub struct CoinGecko {
    pub(crate) client: Client,
}

impl CoinGeckoContract for CoinGecko {
    async fn get_info(&self, _id: &str) -> Result<CryptoInfo> {
        todo!()
    }

    async fn list_cryptocurrencies(&self) -> Result<Vec<SimpleCoin>> {
        let response = self.client.get(COINS_LIST_URL).send().await?.text().await?;
        let coins = serde_json::from_str(&response)?;
        Ok(coins)
    }
}
