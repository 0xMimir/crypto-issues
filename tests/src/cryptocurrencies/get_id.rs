use crate::request::request;
use error::Result;
use reqwest::Method;
use sea_orm::{
    prelude::Uuid, sea_query::OnConflict, DatabaseConnection, EntityTrait, ModelTrait, Set,
};
use store::{
    cryptocurrencies, github_projects, github_repositories, objects::CryptoCurrencyWithRepositories,
};

///
/// Test function for /api/v1/crypto/{id}
///
pub async fn api_v1_crypto_id(sea_pool: &DatabaseConnection) {
    let (github, crypto) = setup(sea_pool).await.unwrap();

    let response: CryptoCurrencyWithRepositories =
        request(format!("/api/v1/crypto/{}", crypto.id), Method::GET, ())
            .await
            .unwrap();

    assert_eq!(response.repositories.len(), 1);

    github.delete(sea_pool).await.unwrap();
    crypto.delete(sea_pool).await.unwrap();
}

async fn setup(
    sea_pool: &DatabaseConnection,
) -> Result<(github_projects::Model, cryptocurrencies::Model)> {
    let github = github_projects::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set("TestGit".to_owned()),
    };

    let github = github_projects::Entity::insert(github)
        .on_conflict(
            OnConflict::column(github_projects::Column::Name)
                .do_nothing()
                .to_owned(),
        )
        .exec_with_returning(sea_pool)
        .await?;

    let crypto = cryptocurrencies::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set("Test coin 1000".to_owned()),
        coingecko_id: Set("test-coin-at-coingecko".to_owned()),
        github: Set(Some(github.id)),
        ..Default::default()
    };

    let crypto = cryptocurrencies::Entity::insert(crypto)
        .on_conflict(OnConflict::default().do_nothing().to_owned())
        .exec_with_returning(sea_pool)
        .await?;

    let github_repo = github_repositories::ActiveModel {
        project: Set(github.id),
        repository_name: Set("good-repo".to_owned()),
        ..Default::default()
    };

    github_repositories::Entity::insert(github_repo)
        .on_conflict(OnConflict::default().do_nothing().to_owned())
        .exec(sea_pool)
        .await?;

    Ok((github, crypto))
}
