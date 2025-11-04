# localsend-rs 0.2.43
CLI implementation of [localsend](https://github.com/localsend/localsend).

## Install
```bash
cargo install --git https://github.com/pepa65/localsend-rs
```

## Usage
### Send
```bash
# send text only
localsend send "text to sent"

# send files
localsend send /path/to/file1 /path/to/file2 ...

# send mixed texts and files
localsend send "text to sent" /path/to/file ...
```

### Receive
```bash
# receive files and save to $(pwd)
localsend receive

# receive files and save to path
localsend receive --dest /path/to/save

# receive all files automatically
localsend receive --quick-save
```

### Help
```
localsend-rs 0.2.43 - CLI implementation of localsend
USAGE: localsend [OPTIONS] <COMMAND>
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
