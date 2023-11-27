use super::contract::DbServiceContract;
use cronus::{Job, Schedule};
use error::Result;
use sdks::github::{data::GithubTopicRepository, GithubContract};
use sea_orm::{ActiveValue::NotSet, Set};
use store::topics_repositories::ActiveModel;

const TOPICS: [&'static str; 4] = ["bitcoin", "cryptocurrency", "ethereum", "monero"];

pub struct GithubTopics<Service: DbServiceContract, Github: GithubContract> {
    service: Service,
    github: Github,
}

impl<
        Service: DbServiceContract + Send + Sync + 'static,
        Github: GithubContract + Send + Sync + 'static,
    > GithubTopics<Service, Github>
{
    ///
    /// Creates GithubRepositoriesCron
    ///
    pub fn new(service: Service, github: Github) -> Self {
        Self { service, github }
    }

    ///
    /// Cron job that runs once a week
    ///
    async fn cron_job(&self) -> Result<()> {
        for topic in TOPICS {
            self.scrape_topic(topic).await;
        }

        self.service.update_projects().await?;
        Ok(())
    }

    ///
    /// Scrape all pages for topic this is always 50 pages
    ///
    async fn scrape_topic(&self, topic: &str) {
        for page in 1..51 {
            if let Err(error) = self.scrape_topic_page(topic, page).await {
                error!("{}", error);
            }
        }
    }

    ///
    /// Scrape page for topic
    ///
    async fn scrape_topic_page(&self, topic: &str, page: u8) -> Result<()> {
        let items = self
            .github
            .get_topic_repositories(topic, page)
            .await?
            .into_iter()
            .map(
                |GithubTopicRepository {
                     name,
                     language,
                     stargazers_count,
                     owner,
                 }| {
                    ActiveModel {
                        repository_name: Set(name),
                        repository_owner: Set(owner),
                        language: Set(language),
                        stargazers_count: Set(stargazers_count),
                        id: NotSet,
                        scraped: NotSet,
                    }
                },
            )
            .collect();

        self.service.upsert_projects(items).await?;
        Ok(())
    }
}

#[async_trait]
impl<Service, Github> Job for GithubTopics<Service, Github>
where
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
