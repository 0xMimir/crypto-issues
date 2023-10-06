use chrono::NaiveDateTime;
use error::Error;
use serde::{de::Error as DeError, Deserialize, Deserializer};

#[derive(Deserialize)]
pub(super) struct GithubRepository {
    pub name: String,
}

impl GithubRepository {
    pub fn into(response: Vec<Self>) -> Vec<String> {
        response.into_iter().map(|gr| gr.name).collect()
    }
}

#[derive(Deserialize)]
pub(super) struct ErrorResponse {
    message: String,
}

impl From<ErrorResponse> for Error {
    fn from(value: ErrorResponse) -> Self {
        if value.message.starts_with("API rate limit exceeded for") {
            return Error::RateLimitExceeded;
        }

        if value.message.starts_with("Not Found") {
            return Error::NotFound;
        }

        if value.message == "Bad credentials" {
            return Error::Unauthorized;
        }

        Self::InternalServer(value.message)
    }
}

#[derive(Deserialize)]
pub struct GithubIssue {
    pub id: i64,
    pub title: String,
    #[serde(rename = "body")]
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: NaiveDateTime,
    pub state: State,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Closed,
    Open,
}

pub fn deserialize_datetime<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<NaiveDateTime, D::Error> {
    let time: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%SZ").map_err(D::Error::custom)
}
