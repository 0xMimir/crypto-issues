use std::sync::Arc;

use lazy_static::lazy_static;
use sea_orm::{sea_query::OnConflict, ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set};
use store::{github_projects, github_repositories, issues};
use support::db_pool::create_db_pool;
use uuid::Uuid;

use crate::{e2e_api::run_e2e_api_tests, helpers::default_github_repo};

mod search;

lazy_static! {
    pub static ref GITHUB_UUID: Uuid = Uuid::new_v4();
}

const PORT: u16 = 1113;

#[adtest::adtest(
    setup = async test_setup,
    cleanup = async test_cleanup
)]
#[serial_test::serial]
async fn projects() {
    let (sea_pool, _) = _setup_;

    run_e2e_api_tests(sea_pool, PORT, test_routes).await;
}

async fn test_setup() -> (Arc<DatabaseConnection>, github_projects::Model) {
    let sea_pool = create_db_pool().await;
    let github = github_projects::ActiveModel {
        id: Set(*GITHUB_UUID),
        name: Set("TestGit".to_owned()),
        profile_type: NotSet,
        url: NotSet,
        followers: NotSet,
    };

    let github = github_projects::Entity::insert(github)
        .on_conflict(
            OnConflict::column(github_projects::Column::Name)
                .do_nothing()
                .to_owned(),
        )
        .exec_with_returning(sea_pool.as_ref())
        .await
        .expect("Error creating github project");

    let github_repo = default_github_repo(github.id);

    let repository = github_repositories::Entity::insert(github_repo)
        .on_conflict(OnConflict::default().do_nothing().to_owned())
        .exec_with_returning(sea_pool.as_ref())
        .await
        .expect("Error creating github repository");

    let issues = issues::ActiveModel {
        id: Default::default(),
        repository: Set(repository.id),
        issue: Set(420),
        title: Set("Test issues".to_owned()),
        description: Default::default(),
        created_at: Set(repository.created_at),
        closed: Set(false),
    };

    issues::Entity::insert(issues)
        .on_conflict(OnConflict::default().do_nothing().to_owned())
        .exec(sea_pool.as_ref())
        .await
        .expect("Error creating issues");

    (sea_pool, github)
}

async fn test_cleanup() {
    let sea_pool = create_db_pool().await;

    github_projects::Entity::delete_by_id(*GITHUB_UUID)
        .exec(sea_pool.as_ref())
        .await
        .expect("Error cleaning up");
}

async fn test_routes() {
    search::api_v1_projects().await
}
