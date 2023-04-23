use structopt::StructOpt;

use super::subcommands::{common::SubCommand, download_cmd::DownloadCmd, upload_cmd::UploadCmd};

const VERSION: &str = "v0.0.3";
const DEFAULT_PATH: &str = "$HOME/.config/nvim/init.vim";

#[derive(StructOpt, Debug)]
#[structopt(about = "Synchronizing for Neovim's configurations", version = VERSION)]
pub enum Vcs {
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
    let cmd = Vcs::from_args();
    let sub_cmd = get_subcommand(cmd);
    sub_cmd.process_cmd();
}

fn get_subcommand(cmd: Vcs) -> Box<dyn SubCommand> {
    match cmd {
        Vcs::Upload { file } => {
            let mut is_default = true;
            if file != DEFAULT_PATH {
                is_default = false;
            }
            Box::new(UploadCmd {
                filename: file,
                use_default_path: is_default,
            })
        }
        Vcs::Download => Box::new(DownloadCmd {}),
    }
}
