use error::Result;
use store::objects::CryptoCurrencyView;

#[async_trait]
pub trait DbRepositoryContract{
    async fn get_cryptocurrencies(&self) -> Result<Vec<CryptoCurrencyView>>;
}

#[async_trait]
pub trait CryptocurrenciesContract{
    async fn get_cryptocurrencies(&self) -> Result<Vec<CryptoCurrencyView>>;
}