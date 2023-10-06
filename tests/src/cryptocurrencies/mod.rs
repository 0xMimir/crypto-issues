use sea_orm::DatabaseConnection;

mod get;

pub async fn test(sea_pool: &DatabaseConnection) {
    get::api_v1_crypto(sea_pool).await
}
