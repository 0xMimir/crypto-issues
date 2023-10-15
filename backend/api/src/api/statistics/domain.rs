use super::contract::{DbRepositoryContract, StatisticsContract};
use store::objects::LanguageCount;
use error::Result;

pub struct Statistics<Repository: DbRepositoryContract> {
    pub repository: Repository,
}

#[async_trait]
impl<Repository> StatisticsContract for Statistics<Repository>
where
    Repository: DbRepositoryContract + Send + Sync,
{
    async fn get_language_counts(&self) -> Result<Vec<LanguageCount>> {
        self.repository.get_language_counts().await
    }
}
