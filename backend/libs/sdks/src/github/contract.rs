use error::Result;

use super::data::{GithubIssue, GithubRepository, GithubTopicRepository, ProfileInfo, RateLimit};

#[async_trait]
pub trait GithubContract {
    ///
    /// Returns repos for github username for page
    ///
    async fn get_repos(&self, username: &str, page: u64) -> Result<Vec<GithubRepository>>;

    ///
    /// Get issues for repository
    ///
    async fn get_issues(
        &self,
        project: &str,
        repository: &str,
        page: u64,
    ) -> Result<Vec<GithubIssue>>;

    ///
    /// Get rate limit
    ///
    async fn get_rate_limit(&self) -> Result<RateLimit>;

    ///
    /// Get user profile
    ///
    async fn get_profile(&self, username: &str) -> Result<ProfileInfo>;

    ///
    /// Get repositories for github topic, max page is 50
    ///
    async fn get_topic_repositories(
        &self,
        topic: &str,
        page: u8,
    ) -> Result<Vec<GithubTopicRepository>>;

    ///
    /// Get repository info
    ///
    async fn get_repository(&self, project: &str, repository: &str) -> Result<GithubRepository>;
}
