use crate::coingecko::data::CryptoInfoResponse;

use super::{data::{CryptoInfo, ErrorResponse}, CoinGeckoContract, SimpleCoin};
use error::Result;
use reqwest::Client;

const COINS_LIST_URL: &str = "https://api.coingecko.com/api/v3/coins/list";

#[derive(Default)]
pub struct CoinGecko {
    pub(crate) client: Client,
}

impl CoinGeckoContract for CoinGecko {
    async fn get_info(&self, id: &str) -> Result<CryptoInfo> {
        let url = format!("https://api.coingecko.com/api/v3/coins/{}?localization=false&tickers=false&market_data=false&community_data=false&sparkline=false", id);
        let response = self.client.get(url).send().await?.text().await?;


        let error = match serde_json::from_str::<CryptoInfoResponse>(&response){
            Ok(response) => return Ok(response.into()),
            Err(error) => error
        };

        
        match serde_json::from_str::<ErrorResponse>(&response){
            Ok(error) => Err(error.into()),
            Err(_) => Err(error.into())
        }
    }

    async fn list_cryptocurrencies(&self) -> Result<Vec<SimpleCoin>> {
        let response = self.client.get(COINS_LIST_URL).send().await?.text().await?;
        let coins = serde_json::from_str(&response)?;
        Ok(coins)
    }
}
