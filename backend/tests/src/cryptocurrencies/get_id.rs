use crate::request::request;
use error::ErrorResponse;
use reqwest::Method;
use store::objects::CryptoCurrencyWithRepositories;
use uuid::Uuid;

///
/// Test function for /api/v1/crypto/{id}
///
pub async fn api_v1_crypto_id(id: Uuid) {
    let response: CryptoCurrencyWithRepositories =
        request(format!("/api/v1/crypto/{}", id), Method::GET, (), ())
            .await
            .unwrap();

    assert_eq!(response.repositories.len(), 1);

    let response: ErrorResponse = request("/api/v1/crypto/not-a-uuid", Method::GET, (), ())
        .await
        .unwrap();

    assert_eq!(response.status, 500);

    let response: ErrorResponse = request(
        "/api/v1/crypto/aac5a965-e14a-472b-a0e4-66bf6f65d39f",
        Method::GET,
        (),
        (),
    )
    .await
    .unwrap();

    assert_eq!(response.status, 404);
}
