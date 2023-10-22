use reqwest::Method;
use store::objects::LanguageCount;

use super::PORT;
use crate::request::request;

const ROUTE: &str = "/api/v1/statistics/languages-count";

pub async fn api_v1_statistics_languages_count() {
    let projects: Vec<LanguageCount> = request(ROUTE, PORT, Method::GET, (), ())
        .await
        .expect("Error getting languages projects");

    assert!(!projects.is_empty());

    let rust = projects
        .into_iter()
        .find(|lc| lc.language == "Rust")
        .unwrap();
    
    assert_eq!(rust.count, 1);
}
