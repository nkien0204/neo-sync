use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    Url,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, time::Duration};

const HTTP_HEADER_GITHUB_ACCESS_TOKEN_KEY: &str = "GITHUB_ACCESS_TOKEN";
const HTTP_HEADER_USER_AGENT_VALUE: &str = "vim-config-sync";
const HTTP_HEADER_ACCEPT_VALUE: &str = "application/vnd.github+json";
const HTTP_HEADER_GITHUB_API_VERSION_KEY: &str = "X-GitHub-Api-Version";
const HTTP_HEADER_GITHUB_API_VERSION_VALUE: &str = "2022-11-28";

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

#[derive(serde::Serialize, Deserialize)]
pub struct GetGistBodyRes {
    pub files: HashMap<String, FilesObject>,
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
            match env::var(HTTP_HEADER_GITHUB_ACCESS_TOKEN_KEY) {
                Ok(v) => v,
                Err(e) => {
                    println!("{}: {}", HTTP_HEADER_GITHUB_ACCESS_TOKEN_KEY, e);
                    panic!("{}", e)
                }
            }
        );
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(HTTP_HEADER_USER_AGENT_VALUE),
        );
        headers.insert(ACCEPT, HeaderValue::from_static(HTTP_HEADER_ACCEPT_VALUE));
        headers.insert(AUTHORIZATION, HeaderValue::try_from(authz_token).unwrap());
        headers.insert(
            HTTP_HEADER_GITHUB_API_VERSION_KEY,
            HeaderValue::from_static(HTTP_HEADER_GITHUB_API_VERSION_VALUE),
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

    pub fn get_gist(&self, id: String) -> Result<Response, Box<dyn std::error::Error>> {
        let update_url = Url::parse(format!("{}/{}", self.url, id).as_str())?;
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        let response = client
            .get(update_url.as_str())
            .headers(self.set_headers())
            .send()?;
        Ok(response)
    }
}
