use super::contract::{DbRepositoryContract, DbServiceContract};
use cronus::{Job, Schedule};
use error::{Error, Result};
use sdks::coingecko::CoinGeckoContract;
use std::time::Duration;
use tokio::time::sleep;

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
        self.service.insert_crypto(cryptocurrencies).await?;
        Ok(())
    }

    ///
    /// Find all cryptocurrencies that don't have neither description, not github, nor gitlab
    ///
    async fn update_info(&self) -> Result<()> {
        let mut cryptocurrencies = self.repository.get_assets_without_info().await?;
        let mut github_ids = self.repository.get_projects().await?;

        while let Some((id, coingecko_id)) = cryptocurrencies.last() {
            let info = match self.coingecko.get_info(coingecko_id).await {
                Ok(info) => info,
                Err(Error::RateLimitExceeded) => {
                    warn!("Rate limit exceeded sleeping for minute");
                    sleep(Duration::from_secs(60)).await;
                    continue;
                }
                Err(Error::NotFoundWithCause(_)) | Err(Error::NotFound) => {
                    if let Err(error) = self.service.delete_crypto(*id).await {
                        error!("{}", error);
                    }
                    continue;
                }
                Err(error) => return Err(error),
            };

            let github = match &info.github {
                Some(project) => match github_ids.get(project) {
                    Some(id) => Some(*id),
                    None => {
                        let github = self.service.create_github(project.to_owned()).await?;
                        github_ids.insert(project.to_owned(), github);
                        Some(github)
                    }
                },
                None => None,
            };

            if let Err(error) = self.service.update_info(*id, info, github).await {
                error!("{}", error);
            }

            cryptocurrencies.pop();
        }
        Ok(())
    }
}

#[async_trait]
impl<Repository, Service, CoinGecko> Job for Info<Repository, Service, CoinGecko>
where
    Repository: DbRepositoryContract + Send + Sync + 'static,
    Service: DbServiceContract + Send + Sync + 'static,
    CoinGecko: CoinGeckoContract + Send + Sync + 'static,
{
    fn schedule(&self) -> Schedule {
        "0 0 0 * * * *".parse().expect("Invalid schedule")
    }
    async fn job(&self) {
        if let Err(error) = self.preform_init().await {
            error!("{}", error);
        }
    }
}
