use sea_orm::prelude::Uuid;
use support::order::Order;
use utoipa::IntoParams;
use validify::Validify;

#[derive(Validify, Deserialize, IntoParams, Debug)]
pub struct SearchRepositoryParams {
    pub order_by: Option<String>,
    pub order: Option<Order>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub language: Option<String>,
    pub repository: Option<String>,
    pub project_id: Option<Uuid>,
    pub project: Option<String>,
    pub archived: Option<bool>,
    pub fork: Option<bool>,
}
