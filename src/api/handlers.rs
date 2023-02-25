use actix_web::{HttpResponse, Result};
use reqwest::header::{HeaderMap, USER_AGENT};
use serde_json::Value;
use rand::prelude::IteratorRandom;

// get meme from Reddit
pub async fn get_mem() -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "reqwest".parse().unwrap());

    let endpoint_url = String::from("https://www.reddit.com/r/ProgrammerHumor/.json");

    let client = reqwest::Client::new();
    let response = client.get(&endpoint_url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    let response: Value = serde_json::from_str(&response)?;

    let posts = response["data"]["children"]
        .as_array()
        .ok_or("No posts found")?;

    let image_posts = posts
        .iter()
        .filter(|post| {
            let is_stickied = post["data"]["stickied"].as_bool().unwrap_or(false);
            let url = post["data"]["url"].as_str().unwrap_or("");
            !is_stickied && url.ends_with(".jpg") || url.ends_with(".png") || url.ends_with(".jpeg")
        })
        .collect::<Vec<_>>();

    let random_post = image_posts.iter().choose(&mut rand::thread_rng()).unwrap();
    let image_url = random_post["data"]["url"].as_str().unwrap();

    Ok(image_url.to_string())
}

// "/api/get_meme"
pub async fn get_image() -> Result<HttpResponse, actix_web::Error> {
    let image_url = get_mem().await?;
    let image_bytes = reqwest::get(&image_url)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))?
        .bytes()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))?
        .to_vec();

    Ok(HttpResponse::Ok().content_type("image/jpeg").body(image_bytes))
}
