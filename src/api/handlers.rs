use actix_web::{HttpResponse, Result};
use rand::prelude::IteratorRandom;

use super::parse_data::fetch_image_posts_cached;

// get random meme
pub async fn get_mem_url() -> Result<String, Box<dyn std::error::Error>> {
    let image_posts = fetch_image_posts_cached().await?;
    let random_post = image_posts.iter().choose(&mut rand::thread_rng()).unwrap();
    let image_url = random_post["data"]["url"].as_str().unwrap();


    Ok(image_url.to_string())
}

// "/api/get_meme"
pub async fn get_image() -> Result<HttpResponse, actix_web::Error> {
    let image_url = get_mem_url().await?;
    let image_bytes = reqwest::get(&image_url)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))?
        .bytes()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))?
        .to_vec();

    Ok(HttpResponse::Ok().content_type("image/jpeg").body(image_bytes))
}
