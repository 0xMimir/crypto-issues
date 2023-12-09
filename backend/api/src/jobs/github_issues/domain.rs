use cronus::{Job, Schedule};
use error::{Error, Result};
use futures::future::join_all;
use sdks::github::GithubContract;
use std::time::Duration;
use store::{
    github_projects::Model as GithubProject, github_repositories::Model as GithubRepository,
};
use tokio::time::sleep;

use super::contract::{DbRepositoryContract, DbServiceContract};

pub struct GithubIssueCron<
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
    > GithubIssueCron<Repository, Service, Github>
{
    ///
    /// Creates `GithubIssueCron`
    ///
    pub fn new(repository: Repository, service: Service, github: Github) -> Self {
        Self {
            repository,
            service,
            github,
        }
    }

    ///
    /// Get all github projects from db and call `handle_project` for every project
    ///
    pub(super) async fn cron_job(&self) -> Result<()> {
        let projects = self.repository.get_projects().await?;
        for project in projects {
            if let Err(error) = self.handle_project(project).await {
                error!("{}", error);
            }
        }
        Ok(())
    }

    ///
    /// Get all repositories for project then call `handle_issues` for every repository
    ///
    async fn handle_project(&self, project: GithubProject) -> Result<()> {
        let repositories = self.repository.get_project_repositories(project.id).await?;

        for repository in repositories {
            if let Err(error) = self.handle_issues(&project.name, repository).await {
                error!("{}", error);
            }
        }
        Ok(())
    }

    ///
    /// Scan first 500 issues
    ///
    async fn handle_issues(&self, project: &str, repository: GithubRepository) -> Result<()> {
        let mut page = 1;

        while page <= 5 {
            let issues = match self
                .github
                .get_issues(project, &repository.repository_name, page)
                .await
            {
                Ok(issues) => issues,
                Err(Error::RateLimitExceeded) => {
                    warn!("Rate limit exceeded sleeping for 10 minutes");
                    sleep(Duration::from_secs(6000)).await;
                    continue;
                }
                Err(error) => return Err(error),
            };

            if issues.is_empty() {
                break;
            }

            let futures = issues
                .into_iter()
                .map(|issue| self.service.create_issues(repository.id, issue));

            let results = join_all(futures).await;
            for result in results {
                result?;
            }

            page += 1;
        }

        Ok(())
    }
}

#[async_trait]
impl<Repository, Service, Github> Job for GithubIssueCron<Repository, Service, Github>
where
    Repository: DbRepositoryContract + Send + Sync + 'static,
    Service: DbServiceContract + Send + Sync + 'static,
    Github: GithubContract + Send + Sync + 'static,
{
    fn schedule(&self) -> Schedule {
        "0 0 0/6 * * * *".parse().expect("Invalid schedule")
    }
    async fn job(&self) {
        if let Err(error) = self.cron_job().await {
            error!("{}", error);
        }
    }
}
