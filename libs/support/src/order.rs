#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Asc,
    Desc,
}

impl From<Order> for sea_orm::Order {
    fn from(value: Order) -> Self {
        match value {
            Order::Asc => Self::Asc,
            Order::Desc => Self::Desc,
        }
    }
}
