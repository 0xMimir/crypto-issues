use serde::Deserialize;
use support::order::Order;
use utoipa::IntoParams;

const ORDER_BY: &[&str] = &[
    "name",
    "repositories",
    "languagesUsed",
    "issues",
    "stargazersCount",
];

#[derive(IntoParams, Deserialize, Debug, Serialize, Validify)]
#[serde(rename_all = "camelCase")]
pub struct SearchGithubProjectParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub order: Option<Order>,
    #[validate(is_in(ORDER_BY))]
    pub order_by: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_vec")]
    pub languages_used: Option<Vec<String>>,
}

fn deserialize_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let languages = match Option::<String>::deserialize(deserializer)? {
        Some(languages) => languages,
        None => return Ok(None),
    };

    Ok(Some(
        languages
            .split(',')
            .map(|language| language.to_owned())
            .collect(),
    ))
}
