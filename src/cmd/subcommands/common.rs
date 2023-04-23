pub const GIST_PREFIX_FILENAME: &str = ".config/neo-sync";

pub trait SubCommand {
    fn process_cmd(&self);
}
