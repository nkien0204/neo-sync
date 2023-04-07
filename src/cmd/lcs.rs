use structopt::StructOpt;

use super::subcommands::{common::SubCommand, download_cmd::DownloadCmd, upload_cmd::UploadCmd};

const VERSION: &str = "v0.0.1";

#[derive(StructOpt, Debug)]
#[structopt(about = "Synchronizing for Lunarvim's configurations", version = VERSION)]
pub enum Lcs {
    #[structopt(about = "upload local config")]
    Upload {
        /// config file
        #[structopt(
            short("f"),
            long("file"),
            default_value = "$HOME/.config/lvim/config.lua"
        )]
        file: String,
    },

    #[structopt(about = "download config")]
    Download,
}

pub fn execute() {
    let cmd = Lcs::from_args();
    let sub_cmd = get_subcommand(cmd);
    sub_cmd.process_cmd();
}

fn get_subcommand(cmd: Lcs) -> Box<dyn SubCommand> {
    match cmd {
        Lcs::Upload { file } => Box::new(UploadCmd { filename: file }),
        Lcs::Download => Box::new(DownloadCmd {}),
    }
}
