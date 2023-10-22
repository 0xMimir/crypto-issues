use reqwest::Method;
use store::objects::RepositoryView;
use uuid::Uuid;

use crate::request::request;

const ROUTE: &str = "/api/v1/repository/{id}";

pub async fn get_id(id: Uuid) {
    let url = ROUTE.replace("{id}", &id.to_string());

    let repository: RepositoryView = request(url, Method::GET, (), ())
        .await
        .expect("Error getting repository");

    assert_eq!(repository.name, "good-repo");
}
