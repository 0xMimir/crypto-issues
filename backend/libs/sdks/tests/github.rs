use error::{Error, Result};
use sdks::github::{Github, GithubContract};

fn client() -> Github {
    let key = config::get("GITHUB_KEY");

    match key.ok() {
        Some(api_key) => Github::new_with_auth(api_key),
        None => Github::default(),
    }
}

#[tokio::test]
async fn test_repos() -> Result<()> {
    let github = client();

    let mut repos = github
        .get_repos("bitcoin", 1)
        .await?
        .into_iter()
        .map(|repo| repo.name)
        .collect::<Vec<_>>();

    repos.sort();
    assert_eq!(repos, vec!["bips", "bitcoin", "libbase58", "libblkmaker"]);

    Ok(())
}

#[tokio::test]
async fn test_issues() -> Result<()> {
    let github = client();

    let issues = github.get_issues("bitcoin", "bitcoin", 1).await?;

    assert!(!issues.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> Result<()> {
    let github = Github::new_with_auth("invalid-key".to_owned());

    let error = github.get_repos("bitcoin", 1).await.unwrap_err();
    assert!(matches!(error, Error::Unauthorized));

    let github = Github::new();
    let error = github
        .get_repos("github-that-does-not-exist", 1)
        .await
        .unwrap_err();
    assert!(matches!(error, Error::NotFoundWithCause(_)));

    Ok(())
}

#[tokio::test]
async fn test_get_topic_repositories() -> Result<()> {
    let github = Github::default();

    let repositories = github.get_topic_repositories("bitcoin", 1).await?;
    assert!(!repositories.is_empty());
    assert_eq!(repositories.get(0).unwrap().name, "bitcoin");
    Ok(())
}
