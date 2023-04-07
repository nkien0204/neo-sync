use super::common::SubCommand;

pub struct DownloadCmd {}

impl SubCommand for DownloadCmd {
    fn process_cmd(&self) {
        println!("download file")
    }
}
