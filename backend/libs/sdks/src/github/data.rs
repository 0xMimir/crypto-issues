use chrono::NaiveDateTime;
use error::Error;
use serde::{de::Error as DeError, Deserialize, Deserializer};
use strum::{Display, EnumString};

#[derive(Deserialize, Debug)]
pub struct GithubRepository {
    pub name: String,
    pub language: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: NaiveDateTime,
    pub archived: bool,
    pub fork: bool,
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
    pub number: i64,
    pub title: String,
    #[serde(rename = "body")]
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: NaiveDateTime,
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

#[derive(Deserialize)]
pub struct RateLimitResponse {
    pub rate: RateLimit,
}

#[derive(Deserialize, Debug)]
pub struct RateLimit {
    pub remaining: u64,
    pub reset: i64,
}

#[derive(Deserialize, Debug)]
pub struct ProfileInfo {
    #[serde(rename = "type")]
    pub profile_type: ProfileType,
    pub followers: i64, // u64 but postgres doesn't support unsigned numbers,
    #[serde(rename = "blog")]
    pub site: Option<String>,
}

#[derive(Deserialize, Debug, Display, EnumString)]
pub enum ProfileType {
    User,
    Organization,
}
