use super::{
    data::{
        ErrorResponse, GithubIssue, GithubRepository, GithubTopicRepository, ProfileInfo,
        RateLimit, RateLimitResponse,
    },
    GithubContract,
};
use error::{Error, Result};
use reqwest::{
    header::{HeaderName, HeaderValue},
    Client, ClientBuilder, IntoUrl,
};
use scraper::{ElementRef, Html};
use serde::de::DeserializeOwned;

#[rustfmt::skip]
mod selectors{
    macro_rules! selector {
        ($name:ident, $expression:expr) => {
            lazy_static::lazy_static! {
                pub static ref $name: scraper::Selector = scraper::Selector::parse($expression).expect("Error creating selector");
            }
        };
    }

    selector!(REPOSITORY, r#"article[class="border rounded color-shadow-small color-bg-subtle my-4"]"#);
    selector!(NAME, r#"div[class="px-3"]>div>div>h3>a[class="Link text-bold wb-break-word"]"#);
    selector!(OWNER, r#"div[class="px-3"]>div>div>h3>a[class="Link"]"#);
    selector!(LANGUAGE, r#"div[class="color-bg-default rounded-bottom-2"]>div[class="p-3"]>ul>li:nth-child(2)>span>span[itemprop="programmingLanguage"]"#);    
    selector!(STARGAZERS, r#"span[class="Counter js-social-count"]"#);
}

pub struct Github {
    client: Client,
}

impl Default for Github {
    fn default() -> Self {
        Self::new()
    }
}
impl Github {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .default_headers(
                [(
                    HeaderName::from_static("user-agent"),
                    HeaderValue::from_static("crypto-issues"),
                )]
                .into_iter()
                .collect(),
            )
            .build()
            .expect("Error creating client");

        Self { client }
    }

    pub fn new_with_auth(api_key: String) -> Self {
        let client = ClientBuilder::new()
            .default_headers(
                [
                    (
                        HeaderName::from_static("user-agent"),
                        HeaderValue::from_static("crypto-issues"),
                    ),
                    (
                        HeaderName::from_static("authorization"),
                        HeaderValue::from_str(format!("Bearer {}", api_key).as_str())
                            .expect("Error parsing HeaderValue"),
                    ),
                ]
                .into_iter()
                .collect(),
            )
            .build()
            .expect("Error creating client");

        Self { client }
    }
}

#[async_trait]
impl GithubContract for Github {
    async fn get_repos(&self, username: &str, page: u64) -> Result<Vec<GithubRepository>> {
        let url = format!("https://api.github.com/users/{username}/repos?page={page}&per_page=100");
        self.get(url).await.map_err(|e| e.add_cause(username))
    }

    async fn get_issues(
        &self,
        project: &str,
        repository: &str,
        page: u64,
    ) -> Result<Vec<GithubIssue>> {
        let url = format!(
            "https://api.github.com/repos/{project}/{repository}/issues?state=all&page={page}&per_page=100"
        );
        self.get(url)
            .await
            .map_err(|e| e.add_cause(format!("{}/{}", project, repository)))
    }

    async fn get_rate_limit(&self) -> Result<RateLimit> {
        let response: RateLimitResponse = self.get("https://api.github.com/rate_limit").await?;
        Ok(response.rate)
    }

    async fn get_profile(&self, username: &str) -> Result<ProfileInfo> {
        let url = format!("https://api.github.com/users/{username}");
        self.get(url).await.map_err(|e| e.add_cause(username))
    }

    async fn get_topic_repositories(
        &self,
        topic: &str,
        page: u8,
    ) -> Result<Vec<GithubTopicRepository>> {
        let url = format!("https://github.com/topics/{topic}?page={page}");
        let html = self.get_raw(url).await?;

        Html::parse_fragment(&html)
            .select(&selectors::REPOSITORY)
            .map(Self::parse_topic_repository)
            .map(|topic| topic.ok_or(Error::InternalServer("Error parsing repository".to_owned())))
            .try_collect()
    }

    async fn get_repository(&self, project: &str, repository: &str) -> Result<GithubRepository> {
        let url = format!("https://api.github.com/repos/{}/{}", project, repository);
        self.get(url).await
    }
}

impl Github {
    async fn get<U, R>(&self, url: U) -> Result<R>
    where
        U: IntoUrl,
        R: DeserializeOwned + 'static,
    {
        let response = self.get_raw(url).await?;
        if let Ok(response) = serde_json::from_str(&response) {
            return Ok(response);
        }

        match serde_json::from_str::<ErrorResponse>(&response) {
            Ok(error) => Err(error.into()),
            Err(_) => Err(Error::InternalServer(response)),
        }
    }

    async fn get_raw<U>(&self, url: U) -> Result<String>
    where
        U: IntoUrl,
    {
        let response = self.client.get(url).send().await?.text().await?;
        Ok(response)
    }

    fn parse_topic_repository<'a>(element: ElementRef<'a>) -> Option<GithubTopicRepository> {
        let name = element
            .select(&selectors::NAME)
            .next()?
            .text()
            .collect::<String>()
            .trim()
            .to_owned();

        let owner = element
            .select(&selectors::OWNER)
            .next()?
            .text()
            .collect::<String>()
            .trim()
            .to_owned();

        let language = element
            .select(&selectors::LANGUAGE)
            .next()
            .map(|x| x.text().collect::<String>().trim().to_owned());

        let stargazers_count = element
            .select(&selectors::STARGAZERS)
            .next()?
            .attr("title")?
            .replace(',', "")
            .parse()
            .ok()?;

        Some(GithubTopicRepository {
            name,
            language,
            stargazers_count,
            owner,
        })
    }
}
