use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection};

pub async fn create_db_pool() -> Arc<DatabaseConnection> {
    let db_url = config::get("DATABASE_URL").expect("DATABASE_URL not set in env");
    let pool = Database::connect(db_url)
        .await
        .expect("Error creating sea pool");

    Arc::new(pool)
}
