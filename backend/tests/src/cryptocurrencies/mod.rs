use std::sync::Arc;

use lazy_static::lazy_static;
use sea_orm::{sea_query::OnConflict, DatabaseConnection, EntityTrait, Set};
use store::{cryptocurrencies, github_projects, github_repositories};
use support::db_pool::create_db_pool;
use uuid::Uuid;

use crate::{e2e_api::run_e2e_api_tests, helpers::default_github_repo};

mod get;
mod get_id;

lazy_static! {
    pub static ref CRYPTO_ID: Uuid = Uuid::new_v4();
    pub static ref GITHUB_ID: Uuid = Uuid::new_v4();
}

const PORT: u16 = 1112;

#[adtest::adtest(
    setup = async test_setup,
    cleanup = async test_cleanup
)]
#[serial_test::serial]
async fn cryptocurrency() {
    let sea_pool = _setup_;

    run_e2e_api_tests(sea_pool, PORT, test_routes).await;
}

async fn test_setup() -> Arc<DatabaseConnection> {
    let sea_pool = create_db_pool().await;

    let github = github_projects::ActiveModel {
        id: Set(*GITHUB_ID),
        name: Set("TestGit".to_owned()),
    };

    let github = github_projects::Entity::insert(github)
        .on_conflict(
            OnConflict::column(github_projects::Column::Name)
                .do_nothing()
                .to_owned(),
        )
        .exec_with_returning(sea_pool.as_ref())
        .await
        .expect("Error creating github");

    let crypto = cryptocurrencies::ActiveModel {
        id: Set(*CRYPTO_ID),
        name: Set("Test coin 1000".to_owned()),
        coingecko_id: Set("test-coin-at-coingecko".to_owned()),
        github: Set(Some(github.id)),
        gitlab: Default::default(),
        description: Default::default(),
    };

    cryptocurrencies::Entity::insert(crypto)
        .on_conflict(OnConflict::default().do_nothing().to_owned())
        .exec_with_returning(sea_pool.as_ref())
        .await
        .expect("Error creating crypto");

    let github_repo = default_github_repo(github.id);

    github_repositories::Entity::insert(github_repo)
        .on_conflict(OnConflict::default().do_nothing().to_owned())
        .exec(sea_pool.as_ref())
        .await
        .expect("Error creating github repository");

    sea_pool
}

async fn test_cleanup() {
    let sea_pool = create_db_pool().await;

    github_projects::Entity::delete_by_id(*GITHUB_ID)
        .exec(sea_pool.as_ref())
        .await
        .expect("Error cleaning up");

    cryptocurrencies::Entity::delete_by_id(*CRYPTO_ID)
        .exec(sea_pool.as_ref())
        .await
        .expect("Error cleaning up");
}

async fn test_routes() {
    get::api_v1_crypto().await;
    get_id::api_v1_crypto_id(*CRYPTO_ID).await;
}
