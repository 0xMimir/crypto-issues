use support::order::Order;

pub struct SearchGithubProjectParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub order: Option<Order>,
    pub order_by: Option<String>,
}
