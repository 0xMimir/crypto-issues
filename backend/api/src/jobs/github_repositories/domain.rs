use std::time::{Duration, Instant};

use super::contract::{DbRepositoryContract, DbServiceContract};
use error::{Error, Result};
use sdks::github::GithubContract;
use store::github_projects::Model;
use tokio::{
    task::JoinHandle,
    time::{interval_at, sleep},
};

pub struct GithubRepositoriesCron<
    Repository: DbRepositoryContract,
    Service: DbServiceContract,
    Github: GithubContract,
> {
    repository: Repository,
    service: Service,
    github: Github,
}

impl<
        Repository: DbRepositoryContract + Send + Sync + 'static,
        Service: DbServiceContract + Send + Sync + 'static,
        Github: GithubContract + Send + Sync + 'static,
    > GithubRepositoriesCron<Repository, Service, Github>
{
    ///
    /// Creates GithubRepositoriesCron
    ///
    pub fn new(repository: Repository, service: Service, github: Github) -> Self {
        Self {
            repository,
            service,
            github,
        }
    }

    ///
    /// Cron job that runs once a week
    ///
    pub async fn cron_job(&self) -> Result<()> {
        let projects = self.repository.get_projects().await?;

        for project in projects {
            let project_id = project.id;
            if let Err(error) = self.download_repos_for_project(project).await {
                match error {
                    Error::NotFoundWithCause(_) | Error::NotFound => {
                        self.service.delete_project(project_id).await?;
                    }
                    _ => error!("{}", error),
                };
            }
        }
        Ok(())
    }

    ///
    /// Download all repos for github project
    ///
    async fn download_repos_for_project(&self, github: Model) -> Result<()> {
        let mut page = 1;

        loop {
            let repositories = match self.github.get_repos(&github.name, page).await {
                Ok(repos) => repos,
                Err(Error::RateLimitExceeded) => {
                    warn!("Rate limit exceeded sleeping for 10 minutes");
                    sleep(Duration::from_secs(6000)).await;
                    continue;
                }
                Err(error) => return Err(error),
            };

            if repositories.is_empty() {
                break;
            }

            self.service
                .create_repository(github.id, repositories)
                .await?;

            page += 1;
        }

        Ok(())
    }

    ///
    /// Spawns tokio task, that waits for a day, then runs once a week
    ///
    pub fn spawn_cron(self) -> JoinHandle<()> {
        info!("Spawning github repositories task");

        tokio::spawn(async move {
            let mut interval = interval_at(
                (Instant::now() + Duration::from_secs(86_400)).into(),
                Duration::from_secs(86_400 * 7),
            );

            loop {
                interval.tick().await;
                if let Err(error) = self.cron_job().await {
                    error!("{}", error);
                }
            }
        })
    }
}
