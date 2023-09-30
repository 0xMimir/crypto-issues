#[derive(Deserialize)]
pub struct SimpleCoin {
    pub id: String,
    pub name: String,
}

pub struct CryptoInfo {
    pub github: Option<String>,
    pub gitlab: Option<String>,
    pub description: Option<String>,
}

impl CryptoInfo {
    pub fn all_none(&self) -> bool {
        self.github.is_none() && self.gitlab.is_none() && self.description.is_none()
    }
}

#[derive(Deserialize)]
pub(crate) struct CryptoInfoResponse {
    pub description: Localization,
    pub links: Links,
}

#[derive(Deserialize)]
pub(crate) struct Localization {
    pub en: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct Links {
    pub repos_url: ReposUrl,
}

#[derive(Deserialize)]
pub(crate) struct ReposUrl {
    pub github: Vec<String>,
}

impl From<CryptoInfoResponse> for CryptoInfo {
    fn from(value: CryptoInfoResponse) -> Self {
        let description = value.description.en;
        let github = value.links.repos_url.extract_github();

        Self {
            github,
            gitlab: None,
            description,
        }
    }
}

impl ReposUrl {
    pub fn extract_github(self) -> Option<String> {
        let url = self.github.into_iter().next()?;
        let url = url.replace("https://github.com/", "");
        Some(url.split_once('/')?.0.to_owned())
    }
}
