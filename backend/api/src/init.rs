use std::sync::{atomic::AtomicBool, Arc};

use api::jobs::{github_repositories::GithubRepositoriesCron, info::Info};
use config::dotenv_init;
use error::Result;
use sdks::{coingecko::CoinGecko, github::Github};
use sea_orm::{Database, DatabaseConnection};
use support::logger::setup_logger;

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() {
    setup_logger();
    dotenv_init();
    let db_url = config::get("DATABASE_URL").unwrap();
    let pool = Database::connect(db_url).await.unwrap();
    let sea_pool = Arc::new(pool);

    let init_finished = Arc::new(AtomicBool::new(false));

    let init_thread = {
        let sea_pool = sea_pool.clone();
        tokio::spawn(async move {
            info_init(sea_pool.clone()).await;
        })
    };

    let github_thread = {
        let init_finished = init_finished.clone();
        tokio::spawn(async move {
            while !init_finished.load(std::sync::atomic::Ordering::Relaxed) {
                if let Err(error) = repo_init(sea_pool.clone()).await {
                    log::error!("{}", error);
                }
            }
        })
    };

    let _ = init_thread.await;
    init_finished.store(true, std::sync::atomic::Ordering::Relaxed);
    let _ = github_thread.await;
}

#[cfg(not(tarpaulin_include))]
async fn info_init(sea_pool: Arc<DatabaseConnection>) {
    let repository = api::jobs::info::infrastructure::PgRepository::new(sea_pool.clone());
    let service = api::jobs::info::infrastructure::PgService::new(sea_pool);
    let coingecko = CoinGecko::default();

    let init = Info::new(repository, service, coingecko);

    init.preform_init().await.unwrap();
}

#[cfg(not(tarpaulin_include))]
async fn repo_init(sea_pool: Arc<DatabaseConnection>) -> Result<()> {
    let repository =
        api::jobs::github_repositories::infrastructure::PgRepository::new(sea_pool.clone());
    let service = api::jobs::github_repositories::infrastructure::PgService::new(sea_pool);
    let github_api_key = config::get("GITHUB_KEY").ok();

    let github = match github_api_key {
        Some(api_key) => Github::new_with_auth(api_key),
        None => Github::default(),
    };

    let init = GithubRepositoriesCron::new(repository, service, github);

    init.cron_job().await
}
