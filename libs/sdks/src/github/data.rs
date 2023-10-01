#[derive(Deserialize)]
pub(crate) struct GithubRepository {
    pub name: String,
}

impl GithubRepository {
    pub fn into(response: Vec<Self>) -> Vec<String> {
        response.into_iter().map(|gr| gr.name).collect()
    }
}
