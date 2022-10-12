## liuliget-bot-rs
[![build](https://github.com/AlanLang/liuliget_bot_rs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/AlanLang/liuliget_bot_rs/actions/workflows/build.yml)

基于 rust 编写的 telegram bot，用于获取琉璃神社的新增文章。

### 使用
```
docker pull alanlang/liuliget-bot-rs:latest
docker run -itd --name liuliget-rs --env TELEGRAM_BOT_TOKEN=<Your token here> TELEGRAM_CHAT_ID=<Your chat id here> --restart=always liuliget-rs
```

### 本地调试
#### 设置 Telegram Bot Token
```
# Unix-like
$ export TELEGRAM_BOT_TOKEN=<Your token here>
$ export TELEGRAM_CHAT_ID=<Your chat id here>

# Windows command line
$ set TELEGRAM_BOT_TOKEN=<Your token here>
$ set TELEGRAM_CHAT_ID=<Your chat id here>

# Windows PowerShell
$ $env:TELEGRAM_BOT_TOKEN=<Your token here>
$ $env:TELEGRAM_CHAT_ID=<Your chat id here>
```
#### debug
```
cargo run
```
