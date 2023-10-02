use sea_orm::{prelude::Uuid, FromQueryResult};

#[derive(Serialize)]
pub struct CryptoCurrencyWithRepositories {
    pub id: Uuid,
    pub name: String,
    pub coingecko_id: String,
    pub github_id: Uuid,
    pub github: String,
    pub repositories: Vec<Repository>,
    pub issues: u64,
}

#[derive(Serialize, FromQueryResult)]
pub struct Repository {
    pub id: Uuid,
    pub repository_name: String,
}
