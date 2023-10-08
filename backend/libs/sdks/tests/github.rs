use error::{Error, Result};
use sdks::github::{Github, GithubContract};

#[tokio::test]
async fn test_repos() -> Result<()> {
    let github = Github::default();

    let mut repos = github.get_repos("bitcoin", 1).await?;
    repos.sort();
    assert_eq!(repos, vec!["bips", "bitcoin", "libbase58", "libblkmaker"]);

    Ok(())
}

#[tokio::test]
async fn test_issues() -> Result<()> {
    let github = Github::default();

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
