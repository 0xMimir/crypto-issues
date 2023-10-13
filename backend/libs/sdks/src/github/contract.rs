use error::Result;

use super::data::{GithubIssue, GithubRepository};

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
}
