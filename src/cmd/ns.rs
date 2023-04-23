use std::env;

use structopt::StructOpt;

use crate::internal::local::gist_management;

use super::subcommands::{
    common::{self, SubCommand},
    download_cmd::DownloadCmd,
    upload_cmd::UploadCmd,
};

const DEFAULT_PATH: &str = "$HOME/.config/nvim/init.vim";

#[derive(StructOpt, Debug)]
#[structopt(about = "Synchronizing for Neovim's configurations")]
pub enum Ns {
    #[structopt(about = "Upload local config")]
    Upload {
        /// config file
        #[structopt(
            short("f"),
            long("file"),
            default_value = DEFAULT_PATH
        )]
        file: String,
    },

    #[structopt(about = "Download config")]
    Download,
}

pub fn execute() {
    let dir_path = format!(
        "{}/{}",
        env::var("HOME").unwrap(),
        common::GIST_PREFIX_FILENAME
    );
    gist_management::create_ns_home(&dir_path).unwrap();

    let cmd = Ns::from_args();
    let sub_cmd = get_subcommand(cmd);
    sub_cmd.process_cmd();
}

fn get_subcommand(cmd: Ns) -> Box<dyn SubCommand> {
    match cmd {
        Ns::Upload { file } => {
            let mut is_default = true;
            if file != DEFAULT_PATH {
                is_default = false;
            }
            Box::new(UploadCmd {
                filename: file,
                use_default_path: is_default,
            })
        }
        Ns::Download => Box::new(DownloadCmd {}),
    }
}
