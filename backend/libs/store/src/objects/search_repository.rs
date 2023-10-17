use chrono::NaiveDateTime;
use sea_orm::{prelude::Uuid, FromQueryResult};

#[derive(FromQueryResult, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchRepository {
    pub id: Uuid,
    pub repository_name: String,
    pub language: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub archived: bool,
    pub project_id: Uuid,
    pub project: String,
    pub url: String,
}
