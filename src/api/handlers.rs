use actix_web::{HttpResponse, Result, web, error::ErrorInternalServerError};
use rand::prelude::IteratorRandom;
use mongodb::{options::ClientOptions, Client, bson::doc};

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


// "/api/create_token/{password}"
pub async fn create_token(pass: web::Path<String>) -> Result<HttpResponse> {
    dotenv::dotenv().ok();

    if Ok(pass.into_inner()) != std::env::var("ROOT_PASS") {
        return Ok(HttpResponse::Ok().body(format!("You did not enter a password or it is incorrect")))
    }

    let mongodb_uri = std::env::var("MONGODB_URI")
        .expect("Failed to get MONGODB_URI variable value from .env file");
    let database_name = std::env::var("DATABASE_NAME")
        .expect("Failed to get DATABASE_NAME variable value from .env file");
    let collection_name = std::env::var("COLLECTION_NAME")
        .expect("Failed to get COLLECTION_NAME variable value from .env file");

    let client_options = ClientOptions::parse(&mongodb_uri)
        .await
        .map_err(|e| ErrorInternalServerError(format!("{}", e)))?;

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&database_name);
    let collection = db.collection(&collection_name);

    let token = uuid::Uuid::new_v4().to_string();

    let document = doc! {
        "token": &token,
        "issued_memes": []
    };

    collection
        .insert_one(document, None)
        .await
        .map_err(|e| ErrorInternalServerError(format!("{}", e)))?;

    Ok(HttpResponse::Ok().body(format!("new token created: {}", token)))
}
