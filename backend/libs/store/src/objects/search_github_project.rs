use sea_orm::{prelude::Uuid, FromQueryResult};

#[derive(Deserialize, Serialize, ToSchema, FromQueryResult)]
pub struct SearchGithubProject {
    pub id: Uuid,
    pub name: String,
    pub repositories: i64,
    pub languages_used: Vec<String>,
    pub issues: i64,
}
