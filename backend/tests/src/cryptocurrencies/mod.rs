use sea_orm::DatabaseConnection;

mod get;
mod get_id;
mod get_issues;

pub async fn test(sea_pool: &DatabaseConnection) {
    get::api_v1_crypto(sea_pool).await;
    get_id::api_v1_crypto_id(sea_pool).await;
    get_issues::api_v1_repository_id_issues(sea_pool).await;
}
