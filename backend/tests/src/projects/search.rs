use reqwest::Method;
use support::pagination::PaginatedProjects;

use super::PORT;
use crate::request::request;

const ROUTE: &str = "/api/v1/projects";

pub async fn api_v1_projects() {
    let projects: PaginatedProjects = request(ROUTE, PORT, Method::GET, (), ())
        .await
        .expect("Error getting paginated projects");

    assert!(!projects.data.is_empty());

    let projects: PaginatedProjects = request(
        ROUTE,
        PORT,
        Method::GET,
        [("languagesUsed", "Rust,Javascript")],
        (),
    )
    .await
    .expect("Error getting paginated projects");

    assert!(projects.data.is_empty());
}
