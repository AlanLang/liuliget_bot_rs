use nipper::Document;
use regex::Regex;
use std::env;

#[derive(Debug)]
pub struct Post {
    pub title: String,
    pub url: String,
    pub description: String,
    pub img: String,
    pub post_type: String,
}

pub async fn get_page() -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let url = match env::var("URL") {
        Ok(val) => val,
        Err(_) => "https://www.hacg.sbs/wp/".to_string(),
    };
    let res = reqwest::get(url).await?.text().await?;
    let document = Document::from(res.as_str());
    let mut posts: Vec<Post> = Vec::new();
    document
        .select(".status-publish")
        .iter()
        .for_each(|athing| {
            let title = athing.select(".entry-title a").text();
            if !title.is_empty() {
                let url = athing.select(".more-link").attr("href").unwrap_or_default();
                let description = athing
                    .select(".entry-content p")
                    .text()
                    .replace(" 继续阅读 →", "");
                let img = athing
                    .select(".entry-content img")
                    .attr("src")
                    .unwrap_or_default();
                let post_type = athing.select(".cat-links").text().replace("发表在 ", "");
                let post = Post {
                    title: title.to_string(),
                    url: url.to_string(),
                    description: description,
                    img: img.to_string(),
                    post_type: post_type.to_string(),
                };
                posts.push(post);
            }
        });
    Ok(posts)
}

pub async fn get_download(url: &String) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?.text().await?;
    let document = Document::from(res.as_str());
    let dom = document.select(".entry-content").text();
    let re = Regex::new(r"[0-9a-fA-F]{40,}.*").unwrap();
    let a: Vec<&str> = re.find_iter(&dom).map(|mat| mat.as_str()).collect();
    for text in &a {
        let mut owned_string: String = "magnet:?xt=urn:btih:".to_owned();
        owned_string.push_str(text);
        return Ok(owned_string);
    }
    Ok(url.to_string())
}
