use error::{Error, Result};
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};

pub async fn request<U, B, R>(url: U, method: Method, body: B) -> Result<R>
where
    U: Into<String>,
    B: Serialize,
    R: DeserializeOwned + 'static,
{
    let client = Client::default();
    let url = format!("http://localhost:1111{}", url.into());
    let request = client.request(method, url).json(&body).send().await?;
    let response_body = request.text().await?;
    serde_json::from_str(&response_body).map_err(Error::from)
}
