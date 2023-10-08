use super::{
    data::{ErrorResponse, GithubIssue, GithubRepository},
    GithubContract,
};
use error::{Error, Result};
use reqwest::{
    header::{HeaderName, HeaderValue},
    Client, ClientBuilder, IntoUrl,
};
use serde::de::DeserializeOwned;

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
                        HeaderValue::from_str(format!("Bearer {}", api_key).as_str()).unwrap(),
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
    async fn get_repos(&self, username: &str, page: u64) -> Result<Vec<String>> {
        let url = format!("https://api.github.com/users/{username}/repos?page={page}&per_page=100");
        let response = self.get(url).await.map_err(|e| e.add_cause(username))?;

        Ok(GithubRepository::into(response))
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
}

impl Github {
    async fn get<U, R>(&self, url: U) -> Result<R>
    where
        U: IntoUrl,
        R: DeserializeOwned + 'static,
    {
        let response = self.client.get(url).send().await?.text().await?;

        if let Ok(response) = serde_json::from_str(&response) {
            return Ok(response);
        }

        match serde_json::from_str::<ErrorResponse>(&response) {
            Ok(error) => Err(error.into()),
            Err(_) => Err(Error::InternalServer(response)),
        }
    }
}
