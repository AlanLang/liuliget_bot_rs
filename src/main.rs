use lib::bot;
use lib::post;
use std::env;
use tokio::time;

async fn timer_to_send(bot: &bot::Bot) {
    let mut interval = time::interval(time::Duration::from_secs(30 * 60));
    let mut post_url: String = String::new();
    loop {
        interval.tick().await;
        let posts = match post::get_page().await {
            Ok(p) => p,
            Err(error) => {
                log::error!("获取页面信息失败: {:?}", error);
                Vec::new()
            }
        };
        let post = posts.first();
        match post {
            Some(x) => {
                if post_url != x.url {
                    log::info!("准备发送文章：{}", x.title);
                    let _ = bot.send_post(x).await; // TODO 处理错误
                    post_url = x.url.to_string();
                }
            }
            None => log::error!("文章解析失败"),
        }
    }
}

#[tokio::main]
async fn main() {
    let bot_token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(val) => val,
        Err(_) => panic!("TELEGRAM_BOT_TOKEN not set"),
    };
    let chart_id = match env::var("TELEGRAM_CHAT_ID") {
        Ok(val) => val,
        Err(_) => panic!("TELEGRAM_CHAT_ID not set"),
    };

    env_logger::init();
    let bot = bot::new(bot_token, chart_id);
    timer_to_send(&bot).await;
}
