use sea_orm::{prelude::Uuid, FromQueryResult};

#[derive(FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryView {
    pub id: Uuid,
    pub name: String,
    pub project_id: Uuid,
    pub project: String,
}
