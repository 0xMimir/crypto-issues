use sea_orm::{prelude::Uuid, FromQueryResult};

#[derive(FromQueryResult, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryView {
    pub id: Uuid,
    pub name: String,
    pub language: Option<String>,
    pub forks_count: i64,
    pub stars_count: i64,
    pub project_id: Uuid,
    pub project: String,
}
