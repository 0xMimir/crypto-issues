use support::order::Order;
use validify::Validify;

#[derive(Validify, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCryptoCurrenciesQuery {
    #[modify(trim)]
    pub search: Option<String>,
    #[modify(trim)]
    pub order_by: Option<String>,
    pub order: Option<Order>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}
