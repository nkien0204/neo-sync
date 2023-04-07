use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub async fn create_gist(
        &self,
        body: CreateGistBody,
    ) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let res = match client.post(self.url.clone()).json(&body).send().await {
            Ok(res) => {println!("res: {:?}", res); res},
            Err(e) => {println!("error: {}", e); return Err(e)}
        };
        println!("{:?}", res);
        Ok(())
    }
}
