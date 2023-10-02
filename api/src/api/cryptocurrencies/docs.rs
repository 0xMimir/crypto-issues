use super::handlers::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_cryptocurrencies, get_issues, get_cryptocurrency_by_id))]
pub struct CryptocurrenciesDocs;
