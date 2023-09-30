use error::Result;

pub trait GithubContract {
    /// 
    /// Returns repos for github username for page
    ///
    async fn get_repos(&self, username: &str, page: u64) -> Result<Vec<String>>;
}
