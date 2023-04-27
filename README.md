# neo-sync
Synchronizing Neovim's configurations

## How to
Support for MacOS, Linux (not test on Windows yet!)
### Setup
```bash
cargo install neo-sync
```

Export `GITHUB_ACCESS_TOKEN` env by your Github Access Token
```bash
export GITHUB_ACCESS_TOKEN=<your-access_token>
```

### Run
```bash
neo-sync <SUBCOMMAND>
```
- To Upload Neovim's config file: 
    ```bash
    neo-sync upload
    ```
    It will try to upload the file on default path (`$HOME/.config/nvim/init.vim`), or provide `-f` option if your file is in other location.

- To Download Neovim's config file: 
    ```bash
    neo-sync download
    ```
    After that, your Neovim's config file should be updated. Default location will be `$HOME/.config/nvim/init.vim` unless you use `-f` option.

For more information, please run `neo-sync -h`.
