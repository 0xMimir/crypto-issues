use super::contract::{DbRepositoryContract, DbServiceContract};
use apis::coingecko::CoinGeckoContract;
use error::Result;

pub struct Init<
    Repository: DbRepositoryContract,
    Service: DbServiceContract,
    CoinGecko: CoinGeckoContract,
> {
    pub(super) repository: Repository,
    pub(super) service: Service,
    pub(super) coingecko: CoinGecko,
}

impl<
        Repository: DbRepositoryContract,
        Service: DbServiceContract,
        CoinGecko: CoinGeckoContract,
    > Init<Repository, Service, CoinGecko>
{
    pub fn new(repository: Repository, service: Service, coingecko: CoinGecko) -> Self {
        Self {
            repository,
            service,
            coingecko,
        }
    }
    pub async fn preform_init(&self) -> Result<()> {
        self.initial_download().await?;
        self.update_info().await?;
        Ok(())
    }

    async fn initial_download(&self) -> Result<()> {
        let cryptocurrencies = self.coingecko.list_cryptocurrencies().await?;

        for crypto in cryptocurrencies {
            if let Err(error) = self.service.insert_crypto(crypto.name, crypto.id).await {
                error!("{}", error);
            }
        }

        Ok(())
    }

    async fn update_info(&self) -> Result<()> {
        let cryptocurrencies = self.repository.get_assets_without_info().await?;

        for (id, coingecko_id) in cryptocurrencies {
            let info = self.coingecko.get_info(&coingecko_id).await?;
            if let Err(error) = self.service.update_info(id, info).await {
                error!("{}", error);
            }
        }

        Ok(())
    }
}
