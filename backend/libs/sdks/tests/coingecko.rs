use error::{Error, Result};
use sdks::coingecko::{CoinGecko, CoinGeckoContract};

#[tokio::test]
async fn test_list() -> Result<()> {
    let coingecko = CoinGecko::default();

    let all_projects = coingecko.list_cryptocurrencies().await?;

    assert!(all_projects.len() > 100);

    assert!(all_projects
        .into_iter()
        .find(|project| project.name == "Bitcoin")
        .is_some());

    Ok(())
}

#[tokio::test]
async fn test_info() -> Result<()> {
    let coingecko = CoinGecko::default();

    let info = coingecko.get_info("bitcoin").await?;
    assert!(!info.all_none());
    assert_eq!(info.github.unwrap(), "bitcoin");

    let error = coingecko.get_info("shit-coin-must-not-exist-402-20").await;

    assert!(matches!(error, Err(Error::NotFoundWithCause(_))));

    Ok(())
}
