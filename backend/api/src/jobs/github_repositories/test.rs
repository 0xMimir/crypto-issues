use sea_orm::{
    prelude::Uuid, sea_query::OnConflict, ActiveValue::NotSet, ColumnTrait, EntityTrait,
    QueryFilter, Set,
};
use store::{github_projects, github_repositories};
use support::db_pool::create_db_pool;

lazy_static::lazy_static! {
    static ref GITHUB_ID: Uuid = Uuid::parse_str("d7c33c88-8938-4998-a348-da1dd95f897a").unwrap();
}

#[adtest(
    setup = async test_setup,
    cleanup = async cleanup
)]
#[serial_test::serial]
async fn test_github_repositories() {
    config::dotenv_init();
    let sea_pool = create_db_pool().await;
    let job = super::create_gr(sea_pool.clone());

    job.cron_job().await.expect("Error while running job");

    let repos = github_repositories::Entity::find()
        .filter(github_repositories::Column::Project.eq(*GITHUB_ID))
        .all(sea_pool.as_ref())
        .await
        .unwrap();

    assert!(repos
        .iter()
        .find(|model| model.repository_name == "bitcoin")
        .is_some());

    assert!(repos
        .iter()
        .find(|model| model.repository_name == "bips")
        .is_some());
}

async fn test_setup() {
    let sea_pool = create_db_pool().await;

    let model = github_projects::ActiveModel {
        name: Set("bitcoin".to_owned()),
        id: Set(*GITHUB_ID),
        profile_type: NotSet,
        url: NotSet,
        followers: Set(0),
    };

    github_projects::Entity::insert(model)
        .on_conflict(
            OnConflict::column(github_projects::Column::Name)
                .do_nothing()
                .to_owned(),
        )
        .exec_without_returning(sea_pool.as_ref())
        .await
        .expect("Error creating model");

    github_repositories::Entity::delete_many()
        .exec(sea_pool.as_ref())
        .await
        .unwrap();
}

async fn cleanup() {
    let sea_pool = create_db_pool().await;
    github_projects::Entity::delete_by_id(*GITHUB_ID)
        .exec(sea_pool.as_ref())
        .await
        .unwrap();
}
