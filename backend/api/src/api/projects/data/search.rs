use support::order::Order;
use utoipa::IntoParams;
use validify::Validify;

#[derive(IntoParams, Deserialize, Validify, Debug)]
pub struct SearchGithubProjectParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub order: Option<Order>,
    pub order_by: Option<String>,
}
