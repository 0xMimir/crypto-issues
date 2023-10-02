use sea_orm::{prelude::Uuid, FromQueryResult};

#[derive(FromQueryResult, Serialize)]
pub struct CryptoCurrencyView {
    pub id: Uuid,
    pub name: String,
    pub coingecko_id: String,
    pub github_id: Uuid,
    pub github: String,
    pub repositories: Vec<String>,
    pub issues: i64,
}
