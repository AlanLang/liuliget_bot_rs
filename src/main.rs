use teloxide::payloads::SendPhotoSetters;
use teloxide::{prelude2::*};
use teloxide::types::InputFile;
use tokio::time;
use std::error::Error;
use reqwest::Url;
mod liuliget;


async fn bot_send_post(bot: &AutoSend<Bot>, chat_id: i64, post: &liuliget::Post) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut post_message: String = "".to_owned();
    post_message.push_str(&post.title);
    post_message.push_str(&post.description);
    post_message.push_str(&post.post_type.trim());
    let download_url = match liuliget::Liuliget::get_download(&post.url).await {
        Ok(p) => p,
        Err(error) => panic!("获取下载地址失败: {:?}", error),
    };
    post_message.push_str("\n");
    post_message.push_str(&download_url);
    let _ = bot.send_photo(chat_id, InputFile::url(Url::parse(&post.img)?))
        .caption(post_message)
        .send().await?;
    Ok(())
}

async fn timer_to_send(bot: AutoSend<Bot>, chat_id: i64) {
    let mut interval = time::interval(time::Duration::from_secs(5 * 60));
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
                    let _ = bot_send_post(&bot, chat_id, x).await;
                    post_url =  x.url.to_string();
                    println!("send success");
                }
            },
            None    => println!("Cannot divide by 0"),
        }

    }
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        tokio::spawn(timer_to_send(bot,message.chat.id));
        respond(())
    })
    .await;
}