use teloxide::{prelude::*, utils::command::BotCommand};
use std::error::Error;
mod liuliget;

#[derive(BotCommand, Debug, PartialEq)]
#[command(rename = "lowercase", description = "支持的命令清单有:")]
enum Command {
    #[command(description = "查看帮助")]
    Help,
    #[command(description = "开启定时监测")]
    Start,
    #[command(description = "停止定时监测")]
    Stop,
    #[command(description = "获取第一页的内容")]
    Refresh,
    #[command(description = "获取当前监测状态")]
    Active,
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        _ => cx.answer("无法识别的命令").await?
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();
    let bot_name: String = String::from("alan_dev_dev");
    let mut liuliget = liuliget::Liuliget::new();
    liuliget.start();
    // TODO: https://github.com/teloxide/teloxide/pull/498
    bot.set_my_commands(vec![
        teloxide::types::BotCommand::new("help", "查看帮助"),
        teloxide::types::BotCommand::new("start", "开启定时监测"),
        teloxide::types::BotCommand::new("stop", "停止定时监测"),
        teloxide::types::BotCommand::new("refresh", "获取第一页的内容"),
        teloxide::types::BotCommand::new("active", "获取当前监测状态"),
    ]).send().await.expect("commands set error");
    teloxide::commands_repl(bot, bot_name, answer).await;
}

async fn handle_message(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
) {
    if let Some(ans) = cx.update.text().map(ToOwned::to_owned) {
        log::info!("re text: {}", ans);
    }
}