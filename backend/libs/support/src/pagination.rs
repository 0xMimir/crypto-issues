#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination<T> {
    pub page: u64,
    pub per_page: u64,
    pub order_by: Vec<String>,
    pub data: Vec<T>,
    pub total_items: u64,
    pub last_page: u64,
}
