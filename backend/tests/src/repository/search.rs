use reqwest::Method;
use store::objects::SearchRepository;
use support::pagination::PaginatedRepositories;

use super::PORT;
use crate::request::request;

const ROUTE: &str = "/api/v1/repository/search";

pub async fn search() -> SearchRepository {
    let mut repositories: PaginatedRepositories = request(ROUTE, PORT, Method::GET, (), ())
        .await
        .expect("Error searching repositories");

    let response: PaginatedRepositories = request(
        ROUTE,
        PORT,
        Method::GET,
        [
            ("language", "DREAM"),
            ("repository", "github.com"),
            ("projectId", "38ecb0bd-7bfe-4919-a6dd-15c948f0ac1f"),
            ("project", "lol"),
            ("fork", "true"),
            ("archived", "true"),
        ],
        (),
    )
    .await
    .expect("Error searching repositories");

    assert!(response.data.is_empty());
    assert_eq!(response.last_page, 0);
    assert_eq!(response.total_items, 0);

    repositories.data.pop().unwrap()
}
