use super::handlers::*;
use store::objects::{CryptoCurrencyView, CryptoCurrencyWithRepositories, Repository};
use support::pagination::PaginatedCryptoCurrencyView;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_cryptocurrencies, get_cryptocurrency_by_id),
    components(schemas(
        CryptoCurrencyWithRepositories,
        Repository,
        PaginatedCryptoCurrencyView,
        CryptoCurrencyView
    ))
)]
pub struct CryptocurrenciesDocs;
