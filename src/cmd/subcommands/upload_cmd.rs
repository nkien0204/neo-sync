use std::{collections::HashMap, env, fs};

use crate::internal::network::http::handler::{CreateGistBody, FilesObject, GithubApiHandler};

use super::common::SubCommand;

pub struct UploadCmd {
    pub filename: String,
    pub use_default_path: bool,
}

impl SubCommand for UploadCmd {
    fn process_cmd(&self) {
        println!("uploading file: {}", self.filename);
        let filename = if self.use_default_path {
            format!("{}/.config/nvim/init.vim", env::var("HOME").unwrap())
        } else {
            self.filename.clone()
        };
        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                println!("error while loading file: {}", e);
                return;
            }
        };
        let handler = GithubApiHandler::new("https://api.github.com/gists".to_string());
        let mut files = HashMap::new();
        files.insert("init.vim".to_string(), FilesObject { content: contents });
        let body = CreateGistBody {
            description: "the configuration for neovim".to_string(),
            public: true,
            files,
        };

        match handler.create_gist(body) {
            Ok(r) => {
                println!("status code: {}", r.status());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        };
    }
}
