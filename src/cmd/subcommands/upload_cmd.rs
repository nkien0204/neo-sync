use std::{collections::HashMap, env, fs};

use crate::{
    cmd::subcommands::common,
    internal::{
        local::gist_management,
        network::http::handler::{FilesObject, GistBody, GistBodyRes, GithubApiHandler},
    },
};

use super::common::SubCommand;

pub struct UploadCmd {
    pub filename: String,
    pub use_default_path: bool,
}

impl SubCommand for UploadCmd {
    fn process_cmd(&self) {
        println!("uploading file: {}", self.filename);
        let vim_filename = if self.use_default_path {
            format!(
                "{}/{}",
                env::var("HOME").unwrap(),
                common::NVIM_CONFIG_FILE_POSTFIX
            )
        } else {
            self.filename.clone()
        };
        let contents = match fs::read_to_string(vim_filename) {
            Ok(c) => c,
            Err(e) => {
                println!("error while loading file: {}", e);
                return;
            }
        };
        let handler = GithubApiHandler::new("https://api.github.com/gists".to_string());
        let mut files = HashMap::new();
        files.insert("init.vim".to_string(), FilesObject { content: contents });
        let body = GistBody {
            description: "the configuration for neovim".to_string(),
            public: true,
            files,
        };

        let gist_filename = format!(
            "{}/{}/gist.txt",
            env::var("HOME").unwrap(),
            common::GIST_PREFIX_FILENAME
        );
        match fs::read_to_string(gist_filename.clone()) {
            Ok(id) => {
                println!("gist existed, trying to update...");
                let res = handler.update_gist(id, body).unwrap();
                println!("{}", res.status());
                return;
            }
            Err(_) => (),
        };

        println!("creating new gist file...");

        match handler.create_gist(body) {
            Ok(r) => {
                println!("status code: {}", r.status());
                if r.status().is_success() {
                    let gist_res: GistBodyRes =
                        serde_json::from_slice(&r.bytes().unwrap()).unwrap();
                    gist_management::save_to_file(&gist_filename, gist_res.id).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        };
    }
}
