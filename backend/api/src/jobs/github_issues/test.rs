use chrono::NaiveDateTime;
use sea_orm::{prelude::Uuid, sea_query::OnConflict, ActiveValue::NotSet, EntityTrait, Set};
use store::{github_projects, github_repositories, issues};
use support::{count::count, db_pool::create_db_pool};

lazy_static::lazy_static! {
    static ref GITHUB_ID: Uuid = Uuid::parse_str("d7c33c88-8938-4998-a348-da1dd95f897a").unwrap();
}

#[adtest(
    setup = async test_setup,
    cleanup = async cleanup
)]
#[serial_test::serial]
async fn test_github_issues() {
    config::dotenv_init();
    let sea_pool = create_db_pool().await;
    let job = super::setup_github_issues(sea_pool.clone());

    let issues = count::<issues::Entity>(&sea_pool)
        .await
        .expect("Error getting issues count");
    job.cron_job().await.expect("Error while running job");

    let new_issues = count::<issues::Entity>(&sea_pool)
        .await
        .expect("Error getting issues count");

    assert!(new_issues > issues);
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

    let model = github_projects::Entity::insert(model)
        .on_conflict(
            OnConflict::column(github_projects::Column::Name)
                .do_nothing()
                .to_owned(),
        )
        .exec_with_returning(sea_pool.as_ref())
        .await
        .expect("Error creating model");

    let model = github_repositories::ActiveModel {
        project: Set(model.id),
        repository_name: Set("bitcoin".to_owned()),
        stargazers_count: Set(0),
        forks_count: Set(0),
        id: Default::default(),
        language: Set(None),
        created_at: Set(NaiveDateTime::default()),
        updated_at: Set(NaiveDateTime::default()),
        archived: Set(false),
        fork: Set(false),
    };

    github_repositories::Entity::insert(model)
        .on_conflict(OnConflict::default().do_nothing().to_owned())
        .exec(sea_pool.as_ref())
        .await
        .expect("Issue inserting github repository");
}

async fn cleanup() {
    let sea_pool = create_db_pool().await;
    github_projects::Entity::delete_by_id(*GITHUB_ID)
        .exec(sea_pool.as_ref())
        .await
        .unwrap();
}
