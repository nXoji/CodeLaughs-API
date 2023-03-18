use reqwest::header::{HeaderMap, USER_AGENT};
use serde_json::Value;
use std::time::{SystemTime, Duration};

// cache for results
struct ResultCache {
    result: Option<Vec<Value>>,
    expiration: Option<SystemTime>,
}

// get memes from reddit
pub async fn fetch_image_posts() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "reqwest".parse().unwrap());

    let endpoint_url = String::from("https://www.reddit.com/r/ProgrammerHumor/.json?limit=1200");

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
            !is_stickied && (url.ends_with(".jpg") || url.ends_with(".png") || url.ends_with(".jpeg"))
        })
        .map(|post| post.clone())
        .collect::<Vec<Value>>();

    Ok(image_posts)
}

// a function that caches the results and returns them if they are already cached
pub async fn fetch_image_posts_cached() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    const CACHE_TIME: u64 = 3600;

    static mut CACHE: ResultCache = ResultCache {
        result: None,
        expiration: None,
    };

    let now = SystemTime::now();
    let mut result_cache = unsafe { &mut CACHE };

    if let Some(expiration) = result_cache.expiration {
        if now < expiration {
            if let Some(result) = &result_cache.result {
                return Ok(result.clone());
            }
        }
    }

    let result = fetch_image_posts().await?;
    result_cache.result = Some(result.clone());
    result_cache.expiration = Some(now + Duration::from_secs(CACHE_TIME));

    Ok(result)
}
