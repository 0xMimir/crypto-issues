use error::{Error, Result};
use sdks::coingecko::{CoinGecko, CoinGeckoContract};

#[tokio::test]
async fn test_list() -> Result<()> {
    let coingecko = CoinGecko::default();

    let all_projects = match coingecko.list_cryptocurrencies().await {
        Ok(r) => r,
        Err(e) => match e {
            Error::RateLimitExceeded => return Ok(()),
            e => return Err(e),
        },
    };

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

    let info = match coingecko.get_info("bitcoin").await {
        Ok(r) => r,
        Err(e) => match e {
            Error::RateLimitExceeded => return Ok(()),
            e => return Err(e),
        },
    };
    assert!(!info.all_none());
    assert_eq!(info.github.unwrap(), "bitcoin");

    let error = match coingecko.get_info("shit-coin-must-not-exist-402-20").await {
        Ok(_) => panic!("This crypto should not exists"),
        Err(e) => match e {
            Error::RateLimitExceeded => return Ok(()),
            e => e,
        },
    };

    assert!(matches!(error, Error::NotFoundWithCause(_)));

    Ok(())
}
