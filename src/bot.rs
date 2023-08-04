use crate::post;
use reqwest::Response;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Bot {
    pub token: String,
    pub chart_id: String,
    pub url: String,
}

pub fn new(token: String, chart_id: String) -> Bot {
    let url = format!("https://api.telegram.org/bot{}/sendPhoto", token);
    Bot {
        token,
        chart_id,
        url,
    }
}

impl Bot {
    pub async fn send_post(&self, post: &post::Post) -> Result<Response, reqwest::Error> {
        let download_url = match post::get_download(&post.url).await {
            Ok(p) => p,
            Err(error) => {
                log::error!("获取下载地址失败: {:?}", error);
                String::from("下载地址获取失败")
            }
        };
        let post_caption = format!(
            "{}\r\n[{}]\r\n{}\r\n{}",
            post.title,
            post.post_type.trim(),
            post.description.trim(),
            download_url
        );

        let mut map = HashMap::new();
        map.insert("chat_id", self.chart_id.as_str());
        map.insert("photo", post.img.as_str());
        map.insert("caption", post_caption.as_str());
        let client = reqwest::Client::new();
        client.post(self.url.as_str()).json(&map).send().await
    }

    pub async fn send_message(&self, message: &str) -> Result<Response, reqwest::Error> {
        let mut map = HashMap::new();
        map.insert("chat_id", self.chart_id.as_str());
        map.insert("text", message);
        let client = reqwest::Client::new();
        client
            .post(format!("https://api.telegram.org/bot{}/sendMessage", self.token).as_str())
            .json(&map)
            .send()
            .await
    }
}
