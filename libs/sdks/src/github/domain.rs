use super::{data::GithubRepository, GithubContract};
use error::{Error, Result};
use reqwest::{Client, StatusCode};

#[derive(Default)]
pub struct Github {
    client: Client,
}

#[async_trait]
impl GithubContract for Github {
    async fn get_repos(&self, username: &str, page: u64) -> Result<Vec<String>> {
        let url = format!("https://api.github.com/users/{username}/repos?page={page}&per_page=100");

        let response = self.client.get(url).send().await?;
        let status = response.status();
        let response = response.text().await?;

        if let Ok(response) = serde_json::from_str::<Vec<GithubRepository>>(&response) {
            return Ok(GithubRepository::into(response));
        }

        match status {
            StatusCode::NOT_FOUND => Err(Error::NotFound(username.to_owned())),
            StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimitExceeded),
            _ => Err(Error::InternalServer(response)),
        }
    }
}
