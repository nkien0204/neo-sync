# neo-sync
Synchronizing Neovim's configurations

## How to
### Setup
```bash
cargo install neo-sync
```
- Export `GITHUB_ACCESS_TOKEN` env by your Github Access Token

### Run
```bash
neo-sync <SUBCOMMAND>
```
- To Upload Neovim's config file: `neo-sync upload`. It will try to upload the file on default path (`$HOME/.config/nvim/init.vim`). Or provide `-f` option if your file is in other location.

For more information, please run `neo-sync -h`.