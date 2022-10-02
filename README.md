## liuliget-bot-rs
[![build](https://github.com/AlanLang/liuliget_bot_rs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/AlanLang/liuliget_bot_rs/actions/workflows/build.yml)
基于 rust 编写的 telegram bot，用于获取琉璃神社的新增文章。

### 使用
```
docker pull alanlang/liuliget-bot-rs:latest
docker run -itd --name liuliget-rs --env TELOXIDE_TOKEN=<Your token here> --restart=always liuliget-rs
```

### 本地调试
#### 设置 Telegram Bot Token
```
# Unix-like
$ export TELOXIDE_TOKEN=<Your token here>

# Windows command line
$ set TELOXIDE_TOKEN=<Your token here>

# Windows PowerShell
$ $env:TELOXIDE_TOKEN=<Your token here>
```
#### debug
```
cargo run
```