use sea_orm::FromQueryResult;

#[derive(Deserialize, Serialize, FromQueryResult, ToSchema)]
pub struct LanguageCount {
    pub language: String,
    pub count: i64,
}
