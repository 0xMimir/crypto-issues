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
