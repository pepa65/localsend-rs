[![version](https://img.shields.io/crates/v/localsnd.svg)](https://crates.io/crates/localsnd)
[![build](https://github.com/pepa65/localsnd/actions/workflows/ci.yml/badge.svg)](https://github.com/pepa65/localsnd/actions/workflows/ci.yml)
[![dependencies](https://deps.rs/repo/github/pepa65/localsnd/status.svg)](https://deps.rs/repo/github/pepa65/localsnd)
[![docs](https://img.shields.io/badge/docs-localsnd-blue.svg)](https://docs.rs/crate/localsnd/latest)
[![license](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/pepa65/localsnd/blob/main/LICENSE)
[![downloads](https://img.shields.io/crates/d/localsnd.svg)](https://crates.io/crates/localsnd)

# localsnd 0.5.8
CLI implementation of [localsend](https://github.com/localsend/localsend).

## Install
### Install standalone single-binary
```sh
wget https://github.com/pepa65/localsnd/releases/download/0.5.8/localsnd
sudo mv localsnd /usr/local/bin
sudo chown root:root /usr/local/bin/localsnd
sudo chmod +x /usr/local/bin/localsnd
```

### Install with cargo
If not installed yet, install a **Rust toolchain**, see <https://www.rust-lang.org/tools/install>

#### Direct from crates.io
`cargo install localsnd`

#### Direct from repo
`cargo install --git https://github.com/pepa65/localsnd`

#### Static build (avoiding GLIBC incompatibilities)
```sh
git clone https://github.com/pepa65/localsnd
cd localsnd
rustup target add x86_64-unknown-linux-musl
cargo rel  # Alias in .cargo/config.toml
```

The binary will be at `target/x86_64-unknown-linux-musl/release/localsnd`

## Usage
### Send
```bash
# send text only
localsnd send "text to sent"

# send files
localsnd send /path/to/file1 /path/to/file2 ...

# send mixed texts and files
localsnd send "text to sent" /path/to/file ...
```

### Receive
```bash
# receive files and save to $(pwd)
localsnd receive

# receive files and save to path
localsnd receive --dest /path/to/save

# receive all files automatically
localsnd receive --quick-save
```

### Help
```
localsnd 0.5.8 - CLI implementation of localsend
USAGE: localsnd [OPTIONS] <COMMAND>
COMMANDS:
  receive  Run as receive server
  send     Run as send client
  help     Print this message or the help of the given subcommand(s)
OPTIONS:
      --alias <ALIAS>          Alias of localsend, use hostname by default [env: LOCALSEND_ALIAS=]
      --multicast <MULTICAST>  Multicast address of localsend [env: LOCALSEND_MULTICAST=] [default: 224.0.0.167]
      --port <PORT>            Port of localsend [env: LOCALSEND_PORT=] [default: 53317]
      --http-port <HTTP_PORT>  Port of localsend http server [env: LOCALSEND_HTTP_PORT=] [default: 53318]
      --nerd                   Use nerd fonts
  -h, --help                   Print help
  -V, --version                Print version
```

## Roadmap
- [x] Settings
    - [x] Device alias
    - [x] Device fingerprint
    - [x] Multicast address
    - [x] Port
    - [ ] Enable https
    - [x] Quick Save
    - [x] Save directory
    - [ ] Non interactive mode
- [x] Discovery
    - [x] Multicast UDP
    - [ ] ~~HTTP(Legacy Mode)~~
- [x] File transfer
    - [x] Send files and texts
    - [ ] Send clipboard data
    - [x] Cancel sending
    - [x] File upload progress bar
    - [x] Fuzzy Select devices
    - [x] Receive files
- [ ] Reverse file transfer
    - [ ] Browser URL
    - [ ] ~~Receive request~~(not in plan)

## Thanks
* [localsend/localsend](https://github.com/localsend/localsend) [#11](https://github.com/localsend/localsend/issues/11)
* [localsend/protocol](https://github.com/localsend/protocol)
* [notjedi/localsend-rs](https://github.com/notjedi/localsend-rs)
* [zpp0196/localsend-rs](https://github.com/zpp0196/localsend-rs)
