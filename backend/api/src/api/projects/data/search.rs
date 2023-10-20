use serde::Deserialize;
use support::order::Order;
use utoipa::IntoParams;

#[derive(IntoParams, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchGithubProjectParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub order: Option<Order>,
    pub order_by: Option<String>,
    #[serde(deserialize_with = "deserialize_vec", default)]
    pub languages_used: Option<Vec<String>>,
}

fn deserialize_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let languages = match String::deserialize(deserializer) {
        Ok(languages) => languages,
        Err(_) => return Ok(None),
    };

    Ok(Some(
        languages
            .split(',')
            .map(|language| language.to_owned())
            .collect(),
    ))
}
