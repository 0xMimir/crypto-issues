use std::time::Duration;

use super::contract::{DbRepositoryContract, DbServiceContract};
use cronus::{Job, Schedule};
use error::Result;
use sdks::github::GithubContract;
use sea_orm::Set;
use store::github_projects::{ActiveModel, Model};
use tokio::time::sleep;

pub struct GithubProjectInfo<
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
    > GithubProjectInfo<Repository, Service, Github>
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
        let projects = self.repository.get_projects().await?.into_iter();

        for project in projects {
            if let Err(error) = self.update_project(project).await {
                error!("{}", error);
            }
        }
        Ok(())
    }

    async fn update_project(&self, model: Model) -> Result<()> {
        for _ in 0..5 {
            match self.github.get_profile(&model.name).await {
                Ok(profile) => {
                    let project = ActiveModel {
                        url: Set(match profile.site.is_empty(){
                            true => Some(profile.site),
                            false => None
                        }),
                        followers: Set(profile.followers),
                        profile_type: Set(Some(profile.profile_type.to_string())),
                        ..model.into()
                    };

                    self.service.update_project(project).await?;

                    break;
                }
                Err(error) => {
                    error!("{}", error);
                }
            };

            sleep(Duration::from_secs(60)).await;
        }

        Ok(())
    }
}

#[async_trait]
impl<Repository, Service, Github> Job for GithubProjectInfo<Repository, Service, Github>
where
    Repository: DbRepositoryContract + Send + Sync + 'static,
    Service: DbServiceContract + Send + Sync + 'static,
    Github: GithubContract + Send + Sync + 'static,
{
    fn schedule(&self) -> Schedule {
        "0 0 0 * * Mon".parse().expect("Invalid schedule")
    }
    async fn job(&self) {
        if let Err(error) = self.cron_job().await {
            error!("{}", error);
        }
    }
}
