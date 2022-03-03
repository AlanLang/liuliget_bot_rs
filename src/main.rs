use teloxide::payloads::SendPhotoSetters;
use teloxide::{prelude2::*, utils::command::BotCommand};
use teloxide::types::InputFile;
use tokio::time;
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

async fn bot_send_post(bot: &AutoSend<Bot>, chat_id: i64, post: &liuliget::Post) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut post_message: String = "".to_owned();
    post_message.push_str(&post.title);
    post_message.push_str(&post.description);
    post_message.push_str(&post.post_type.trim());
    let _ = bot.send_photo(chat_id, InputFile::url(Url::parse(&post.img)?))
        .caption(post_message)
        .send().await?;
    Ok(())
}

async fn timer_to_send(bot: AutoSend<Bot>, chat_id: i64) {
    let mut interval = time::interval(time::Duration::from_secs(10 * 60 * 60));
    let mut post_url: String = String::new();
    loop {
        interval.tick().await;
        let posts = match liuliget::Liuliget::get_page().await {
            Ok(p) => p,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let post = posts.first();
        match post {
            Some(x) => {
                if post_url != x.url {
                    let _ = bot_send_post(&bot, chat_id, x);
                    post_url =  x.url.to_string();
                }
            },
            None    => println!("Cannot divide by 0"),
        }

    }
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
                let mut post_message: String = "".to_owned();
                post_message.push_str(&post.title);
                post_message.push_str(&post.description);
                post_message.push_str(&post.post_type.trim());
                bot.send_photo(message.chat.id, InputFile::url(Url::parse(&post.img)?))
                    .caption(post_message)
                    .send().await?;
            }
            bot.send_message(message.chat.id,"finish").await?
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
    tokio::spawn(timer_to_send(bot,817392195));
    // teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}