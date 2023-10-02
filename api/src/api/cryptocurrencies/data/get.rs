use support::order::Order;
use validify::Validify;

#[derive(Validify, Deserialize, Serialize)]
pub struct GetCryptoCurrenciesQuery {
    #[modify(trim)]
    pub search: Option<String>,
    #[modify(trim)]
    pub order_by: Option<String>,
    pub order: Option<Order>,
}
