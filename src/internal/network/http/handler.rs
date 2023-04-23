use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Url,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, time::Duration};

pub struct GithubApiHandler {
    url: String,
}

#[derive(serde::Serialize, Deserialize)]
pub struct GistBody {
    pub description: String,
    pub files: HashMap<String, FilesObject>,
    pub public: bool,
}

#[derive(serde::Serialize, Deserialize)]
pub struct GistBodyRes {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct FilesObject {
    pub content: String,
}

impl GithubApiHandler {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    fn set_headers(&self) -> HeaderMap {
        let authz_token = format!(
            "Bearer {}",
            match env::var("GITHUB_ACCESS_TOKEN") {
                Ok(v) => v,
                Err(e) => {
                    println!("GITHUB_ACCESS_TOKEN: {}", e);
                    panic!("{}", e)
                }
            }
        );
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("vim-config-sync"));
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert("Authorization", HeaderValue::try_from(authz_token).unwrap());
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_static("2022-11-28"),
        );
        headers
    }

    pub fn create_gist(&self, body: GistBody) -> Result<Response, Box<dyn std::error::Error>> {
        let serial_body = serde_json::to_string(&body)?;
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        let response = client
            .post(self.url.clone())
            .headers(self.set_headers())
            .body(serial_body)
            .send()?;
        Ok(response)
    }

    pub fn update_gist(
        &self,
        id: String,
        body: GistBody,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let update_url = Url::parse(format!("{}/{}", self.url, id).as_str())?;
        let serial_body = serde_json::to_string(&body)?;
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        let response = client
            .patch(update_url.as_str())
            .headers(self.set_headers())
            .body(serial_body)
            .send()?;
        Ok(response)
    }
}
