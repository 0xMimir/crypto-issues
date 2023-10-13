use sea_orm::{prelude::Uuid, FromQueryResult};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoCurrencyWithRepositories {
    pub id: Uuid,
    pub name: String,
    pub coingecko_id: String,
    pub github_id: Uuid,
    pub github: String,
    pub repositories: Vec<Repository>,
    pub issues: u64,
}

#[derive(Serialize, FromQueryResult, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: Uuid,
    pub repository_name: String,
    pub language: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
}
