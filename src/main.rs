use teloxide::{prelude::*};
use teloxide::types::InputFile;
use tokio::time;
use std::error::Error;
use reqwest::Url;
use liuliget_bot::post;
use liuliget_bot::command::{self, Command};
use teloxide::utils::command::BotCommands;

async fn bot_send_post(bot: &AutoSend<Bot>, chat_id: ChatId, post: &post::Post) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut post_message: String = "".to_owned();
    post_message.push_str(&post.title);
    post_message.push_str(&post.description);
    post_message.push_str(&post.post_type.trim());

    let download_url = match post::get_download(&post.url).await {
        Ok(p) => p,
        Err(error) => panic!("获取下载地址失败: {:?}", error),
    };
    post_message.push_str("\n");
    post_message.push_str(&download_url);
    let url = Url::parse(&post.img).unwrap();
    let photo = InputFile::url(url);
    log::info!("发送文章给:{}, 标题为:{}", chat_id, post.title);
    let _ = bot.send_photo(chat_id, photo)
        .caption(post_message)
        .send().await?;
    Ok(())
}

async fn timer_to_send(bot: AutoSend<Bot>, chat_id: ChatId) {
    let mut interval = time::interval(time::Duration::from_secs(5 * 60));
    let mut post_url: String = String::new();
    loop {
        interval.tick().await;
        let posts = match post::get_page().await {
            Ok(p) => p,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let post = posts.first();
        log::info!("文章标题：{}", post.unwrap().title);
        match post {
            Some(x) => {
                if post_url != x.url {
                    let _ = bot_send_post(&bot, chat_id, x).await;
                    post_url =  x.url.to_string(); 
                } else {
                    log::info!("文章[{}]已经发送过了，不再发送", x.title);
                }
            },
            None    => log::info!("Cannot divide by 0"),
        }

    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let bot = Bot::from_env().auto_send();
    // let chats: Vec<ChatId> = Vec::new();
    teloxide::commands_repl(bot, answer, command::Command::ty()).await;
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Start => {
            bot.send_message(message.chat.id, "订阅成功").await?;
            tokio::spawn(timer_to_send(bot,message.chat.id));
        }
    };

    Ok(())
}