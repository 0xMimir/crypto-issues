use sea_orm::prelude::DateTime;
use sea_orm::prelude::Uuid;
use sea_orm::FromQueryResult;

#[derive(Clone, Debug, ToSchema, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct GithubIssue {
    pub id: Uuid,
    pub repository: Uuid,
    pub issue: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub closed: bool,
}
