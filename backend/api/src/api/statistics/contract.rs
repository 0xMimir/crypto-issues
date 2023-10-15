use error::Result;
use store::objects::LanguageCount;

#[async_trait]
pub trait DbRepositoryContract {
    ///
    /// Get count of repositories using language
    ///
    async fn get_language_counts(&self) -> Result<Vec<LanguageCount>>;
}

#[async_trait]
pub trait StatisticsContract {
    ///
    /// Get count of repositories using language
    ///
    async fn get_language_counts(&self) -> Result<Vec<LanguageCount>>;
}
