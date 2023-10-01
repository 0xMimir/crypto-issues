use std::sync::Arc;

use api::jobs::{github_repositories::GithubRepositoriesCron, info::Info};
use config::dotenv_init;
use sdks::{coingecko::CoinGecko, github::Github};
use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .init();

    dotenv_init();
    let db_url = config::get("DATABASE_URL").unwrap();
    let pool = Database::connect(db_url).await.unwrap();
    let sea_pool = Arc::new(pool);

    info_init(sea_pool.clone()).await;
    repo_init(sea_pool).await;
}

async fn info_init(sea_pool: Arc<DatabaseConnection>) {
    let repository = api::jobs::info::infrastructure::PgRepository::new(sea_pool.clone());
    let service = api::jobs::info::infrastructure::PgService::new(sea_pool);
    let coingecko = CoinGecko::default();

    let init = Info::new(repository, service, coingecko);

    init.preform_init().await.unwrap();
}

async fn repo_init(sea_pool: Arc<DatabaseConnection>) {
    let repository =
        api::jobs::github_repositories::infrastructure::PgRepository::new(sea_pool.clone());
    let service = api::jobs::github_repositories::infrastructure::PgService::new(sea_pool);
    let github = Github::default();

    let init = GithubRepositoriesCron::new(repository, service, github);

    init.cron_job().await.unwrap();
}
