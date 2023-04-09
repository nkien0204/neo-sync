use reqwest::{blocking::Client, header::{HeaderMap, USER_AGENT}};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

pub struct GithubApiHandler {
    url: String,
}

#[derive(serde::Serialize, Deserialize)]
pub struct CreateGistBody {
    pub description: String,
    pub files: HashMap<String, FilesObject>,
    pub public: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FilesObject {
    pub content: String,
}

impl GithubApiHandler {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    fn set_headers() -> HeaderMap {
        let headers = HashMap::new();
        headers.insert(USER_AGENT, HeaderValue::from)
    }

    pub fn create_gist(&self, body: CreateGistBody) -> Result<String, Box<dyn std::error::Error>> {
        let serial_body = serde_json::to_string(&body)?;
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        let response = client
            .post(self.url.clone())
            .header("Content-Type", "application/json")
            .headers()   "User-Agent", "lcs",
            .body(serial_body)
            .send()?;
        println!("status code: {}", response.status());
        Ok(response.text()?)
    }
}
