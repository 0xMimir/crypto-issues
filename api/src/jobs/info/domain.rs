use std::time::{Duration, Instant};

use super::contract::{DbRepositoryContract, DbServiceContract};
use error::{Error, Result};
use sdks::coingecko::CoinGeckoContract;
use tokio::{
    task::JoinHandle,
    time::{interval_at, sleep},
};

pub struct Info<
    Repository: DbRepositoryContract,
    Service: DbServiceContract,
    CoinGecko: CoinGeckoContract,
> {
    pub(super) repository: Repository,
    pub(super) service: Service,
    pub(super) coingecko: CoinGecko,
}

impl<
        Repository: DbRepositoryContract + Send + Sync + 'static,
        Service: DbServiceContract + Send + Sync + 'static,
        CoinGecko: CoinGeckoContract + Send + Sync + 'static,
    > Info<Repository, Service, CoinGecko>
{
    ///
    /// Creates Info
    ///
    pub fn new(repository: Repository, service: Service, coingecko: CoinGecko) -> Self {
        Self {
            repository,
            service,
            coingecko,
        }
    }

    ///
    /// This seed database with cryptocurrencies from coingecko
    ///
    pub async fn preform_init(&self) -> Result<()> {
        self.initial_download().await?;
        self.update_info().await?;
        Ok(())
    }

    ///
    /// Downloads all crypto currencies from coingecko
    ///
    async fn initial_download(&self) -> Result<()> {
        let cryptocurrencies = self.coingecko.list_cryptocurrencies().await?;

        for crypto in cryptocurrencies {
            if let Err(error) = self.service.insert_crypto(crypto.name, crypto.id).await {
                error!("{}", error);
            }
        }

        Ok(())
    }

    ///
    /// Find all cryptocurrencies that don't have neither description, not github, nor gitlab
    ///
    async fn update_info(&self) -> Result<()> {
        let mut cryptocurrencies = self.repository.get_assets_without_info().await?;

        while let Some((id, coingecko_id)) = cryptocurrencies.last() {
            let info = match self.coingecko.get_info(&coingecko_id).await {
                Ok(info) => info,
                Err(Error::RateLimitExceeded) => {
                    sleep(Duration::from_secs(60)).await;
                    continue;
                }
                Err(error) => return Err(error),
            };

            if let Err(error) = self.service.update_info(*id, info).await {
                error!("{}", error);
            }

            cryptocurrencies.pop();
        }
        Ok(())
    }
    ///
    /// Spawns tokio task, that waits for a day, then runs once a day
    ///
    pub fn spawn_cron(self) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = interval_at(
                (Instant::now() + Duration::from_secs(86_400)).into(),
                Duration::from_secs(86_400),
            );

            loop {
                interval.tick().await;
                if let Err(error) = self.preform_init().await {
                    error!("{}", error);
                }
            }
        })
    }
}
