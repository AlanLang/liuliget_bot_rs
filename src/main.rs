use teloxide::{prelude2::*, utils::command::BotCommand};
use teloxide::types::InputFile;
use std::error::Error;
use reqwest::Url;
mod liuliget;

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "查看帮助")]
    Help,
    #[command(description = "开始使用")]
    Start,
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut users: Vec<i64> = Vec::new();

    match command {
        Command::Help => bot.send_message(message.chat.id, Command::descriptions()).await?,
        Command::Start => {
            users.push(message.chat.id);
            let posts = match liuliget::Liuliget::get_page().await {
                Ok(p) => p,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };
            for post in &posts {
                println!("{}", &post.title);
                println!("{}", &post.img);
                bot.send_photo(message.chat.id, InputFile::url(Url::parse(&post.img)?)).await?;
            }
            bot.send_message(message.chat.id,"finish").await?
            // // let posts = liuliget::Liuliget::get_page().await;
            // if let Ok(post) = liuliget::Liuliget::get_page().await {
            //     bot.send_message(message.chat.id,"success").await?
            // } else {
            //     bot.send_message(message.chat.id,"获取异常").await?
            // }
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    bot.set_my_commands(vec![
        teloxide::types::BotCommand::new("help", "查看帮助"),
        teloxide::types::BotCommand::new("start", "开始使用"),
    ]).send().await.expect("commands set error");

    teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}