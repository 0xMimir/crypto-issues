use crate::coingecko::data::{CryptoInfoResponse, SimpleError};

use super::{
    data::{CryptoInfo, ErrorResponse},
    CoinGeckoContract, SimpleCoin,
};
use error::{Error, Result};
use reqwest::{Client, IntoUrl, StatusCode};
use serde::de::DeserializeOwned;

const COINS_LIST_URL: &str = "https://api.coingecko.com/api/v3/coins/list";

#[derive(Default)]
pub struct CoinGecko {
    pub(crate) client: Client,
}

impl CoinGecko {
    async fn get<U, R>(&self, url: U) -> Result<R>
    where
        U: IntoUrl + Send + Sync,
        R: DeserializeOwned + 'static,
    {
        let response = self.client.get(url).send().await?;
        let status = response.status();

        match status {
            StatusCode::NOT_FOUND => return Err(Error::NotFound),
            _ => (),
        };

        let response = response.text().await?;

        let error = match serde_json::from_str(&response) {
            Ok(response) => return Ok(response),
            Err(error) => error,
        };

        if let Ok(error) = serde_json::from_str::<ErrorResponse>(&response) {
            return Err(error.into());
        }

        if response.contains("Throttled") {
            return Err(Error::RateLimitExceeded);
        }

        match serde_json::from_str::<SimpleError>(&response) {
            Ok(error) => Err(error.into()),
            Err(_) => {
                error!("{}", response);
                Err(error.into())
            }
        }
    }
}

#[async_trait]
impl CoinGeckoContract for CoinGecko {
    async fn get_info(&self, id: &str) -> Result<CryptoInfo> {
        let url = format!("https://api.coingecko.com/api/v3/coins/{}?localization=false&tickers=false&market_data=false&community_data=false&sparkline=false", id);
        let response = self
            .get::<_, CryptoInfoResponse>(url)
            .await
            .map_err(|e| e.add_cause(id))?;

        Ok(response.into())
    }

    async fn list_cryptocurrencies(&self) -> Result<Vec<SimpleCoin>> {
        self.get(COINS_LIST_URL).await
    }
}
