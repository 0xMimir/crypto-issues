use crate::request::request;
use reqwest::Method;
use store::objects::CryptoCurrencyView;
use support::pagination::Pagination;

const ROUTE: &str = "/api/v1/crypto";

///
/// Test function for /api/v1/crypto
///
pub async fn api_v1_crypto() {
    let response: Pagination<CryptoCurrencyView> =
        request(ROUTE, Method::GET, (), ()).await.unwrap();

    assert!(!response.data.is_empty());

    let response: Pagination<CryptoCurrencyView> =
        request(ROUTE, Method::GET, [("search", "oooooogggggaaaaa")], ())
            .await
            .unwrap();

    assert!(response.data.is_empty());
}
