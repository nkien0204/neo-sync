use std::collections::HashMap;

use crate::internal::network::http::handler::{GithubApiHandler, CreateGistBody, FilesObject};

use super::common::SubCommand;

pub struct UploadCmd {
    pub filename: String,
}

impl SubCommand for UploadCmd {
    fn process_cmd(&self) {
        println!("upload file: {}", self.filename);
        let handler = GithubApiHandler::new("https://api.github.com/gists".to_string());
        let mut files = HashMap::new();
        files.insert("config.lua".to_string(), FilesObject{content: "hello world".to_string()});
        let body = CreateGistBody {
            description: "test".to_string(),
            public: false,
            files
        };

        _ = handler.create_gist(body);
    }
}
