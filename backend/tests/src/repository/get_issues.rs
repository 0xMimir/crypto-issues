use reqwest::Method;
use support::pagination::PaginatedGithubIssue;
use uuid::Uuid;

use crate::request::request;

const ROUTE: &str = "/api/v1/repository/{id}/issues";

pub async fn get_issues(id: Uuid) {
    let url = ROUTE.replace("{id}", id.to_string().as_str());

    let issues: PaginatedGithubIssue = request(&url, Method::GET, (), ())
        .await
        .expect("Error getting issues");

    assert_eq!(issues.total_items, 1);
    assert_eq!(issues.data.len(), 1);

    let issues: PaginatedGithubIssue = request(
        url,
        Method::GET,
        [("closed", "true"), ("search", "Ricky")],
        (),
    )
    .await
    .expect("Error getting issues");

    assert_eq!(issues.total_items, 0);
    assert_eq!(issues.data.len(), 0);
}
