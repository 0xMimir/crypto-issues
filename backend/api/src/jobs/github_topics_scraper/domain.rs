use super::contract::{DbRepositoryContract, DbServiceContract};
use cronus::{Job, Schedule};
use error::Result;
use sdks::github::{
    data::{GithubRepository, ProfileInfo},
    GithubContract,
};
use sea_orm::{prelude::Uuid, ActiveValue::NotSet, Set};
use store::{github_projects, github_repositories};

pub struct GithubTopicsScraper<
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
    > GithubTopicsScraper<Repository, Service, Github>
{
    ///
    /// Creates GithubTopicsScraper
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
    async fn cron_job(&self) -> Result<()> {
        self.handle_projects().await?;
        self.handle_repositories().await?;
        self.service.update_projects().await?;
        Ok(())
    }

    ///
    /// Download unscraped repositories
    /// 
    async fn handle_repositories(&self) -> Result<()> {
        let repositories = self.repository.get_unscraped_repositories().await?;

        for (project_name, repository_name, project_id) in repositories {
            if let Err(error) = self
                .handle_repository(project_name, repository_name, project_id)
                .await
            {
                error!("{}", error);
            }
        }

        Ok(())
    }

    ///
    /// Download repository and store it in db
    /// 
    async fn handle_repository(
        &self,
        project_name: String,
        repository_name: String,
        project_id: Uuid,
    ) -> Result<()> {
        let GithubRepository {
            name,
            language,
            stargazers_count,
            forks_count,
            created_at,
            updated_at,
            archived,
            fork,
        } = self
            .github
            .get_repository(&project_name, &repository_name)
            .await?;

        let repository = github_repositories::ActiveModel {
            id: NotSet,
            project: Set(project_id),
            repository_name: Set(name),
            language: Set(language),
            stargazers_count: Set(stargazers_count),
            forks_count: Set(forks_count),
            created_at: Set(created_at),
            updated_at: Set(updated_at),
            archived: Set(archived),
            fork: Set(fork),
        };

        self.service.insert_repository(repository).await?;
        Ok(())
    }

    ///
    /// Download unscraped projects
    /// 
    async fn handle_projects(&self) -> Result<()> {
        let projects = self.repository.get_unscraped_projects().await?;
        for project in projects {
            if let Err(error) = self.handle_project(project).await {
                error!("{}", error);
            }
        }

        Ok(())
    }

    ///
    /// Download project and store it in db
    /// 
    async fn handle_project(&self, project: String) -> Result<()> {
        let ProfileInfo {
            profile_type,
            followers,
            site,
        } = self.github.get_profile(&project).await?;

        let project = github_projects::ActiveModel {
            id: NotSet,
            name: Set(project),
            profile_type: Set(Some(profile_type.to_string())),
            url: Set(match site.is_empty() {
                true => None,
                false => Some(site),
            }),
            followers: Set(followers),
        };

        self.service.insert_project(project).await
    }
}

#[async_trait]
impl<Repository, Service, Github> Job for GithubTopicsScraper<Repository, Service, Github>
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
