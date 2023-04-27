use std::{
    env, fs,
    io::{self, Write},
};

use crate::{
    cmd::subcommands::common,
    internal::{
        local::gist_management,
        network::http::handler::{GetGistBodyRes, GithubApiHandler},
    },
};

use super::common::SubCommand;

pub struct DownloadCmd {
    pub filename: String,
    pub use_default_path: bool,
}

impl SubCommand for DownloadCmd {
    fn process_cmd(&self) {
        println!("downloading file: {}", self.filename);
        let config_filename = if self.use_default_path {
            format!(
                "{}/{}",
                env::var("HOME").unwrap(),
                common::NVIM_CONFIG_FILE_POSTFIX
            )
        } else {
            self.filename.clone()
        };

        let handler = GithubApiHandler::new("https://api.github.com/gists".to_string());
        let gist_filename = format!(
            "{}/{}/gist.txt",
            env::var("HOME").unwrap(),
            common::GIST_PREFIX_FILENAME
        );
        let id = match fs::read_to_string(gist_filename.clone()) {
            Ok(id) => {
                println!("found gist id from file: {}", id);
                id
            }
            Err(_) => {
                print!("could not get gist id from file, please fill your gist id here: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                input
            }
        };
        let res = handler.get_gist(id).unwrap();
        println!("{}", res.status());
        if !res.status().is_success() {
            return;
        }

        let gist_res: GetGistBodyRes = serde_json::from_slice(&res.bytes().unwrap()).unwrap();
        let file_obj = gist_res.files.get(&String::from("init.vim")).unwrap();

        gist_management::save_to_file(&config_filename, file_obj.content.clone()).unwrap();
    }
}
