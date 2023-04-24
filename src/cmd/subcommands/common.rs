pub const GIST_PREFIX_FILENAME: &str = ".config/neo-sync";
pub const NVIM_CONFIG_FILE_POSTFIX: &str = ".config/nvim/init.vim";

pub trait SubCommand {
    fn process_cmd(&self);
}
