use store::objects::{CryptoCurrencyView, GithubIssue, SearchRepository};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[aliases(
    PaginatedCryptoCurrencyView = Pagination<CryptoCurrencyView>,
    PaginatedGithubIssue = Pagination<GithubIssue>,
    PaginatedRepositories = Pagination<SearchRepository>
)]
pub struct Pagination<T> {
    pub page: u64,
    pub per_page: u64,
    pub order_by: Vec<String>,
    pub data: Vec<T>,
    pub total_items: u64,
    pub last_page: u64,
}
