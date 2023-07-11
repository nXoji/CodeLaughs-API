use actix_web::{HttpResponse, HttpRequest, Result, web, error::ErrorInternalServerError};
use rand::prelude::IteratorRandom;
use mongodb::{options::{ClientOptions, FindOneOptions}, Client, bson::{doc, Bson}};

use super::parse_data::fetch_image_posts_cached;

// get random meme url
pub async fn get_mem_url() -> Result<String, Box<dyn std::error::Error>> {
    let image_posts = fetch_image_posts_cached().await?;
    let random_post = image_posts.iter().choose(&mut rand::thread_rng()).unwrap();
    let image_url = random_post["data"]["url"].as_str().unwrap();

    Ok(image_url.to_string())
}

// "/api/get_meme"
pub async fn get_image(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let api_key = req.headers().get("X-API-Key");

    match api_key {
        Some(key) => {
            let key_value = key.to_str().unwrap();

            dotenv::dotenv().ok();

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
            let collection = db.collection::<mongodb::bson::Document>(&collection_name);

            let filter = doc! { "token": key_value };
            let options = FindOneOptions::default();
            let result = collection.find_one(filter, options).await;

            match result {
                Ok(Some(_)) => (),
                Ok(None) => return Ok(HttpResponse::Unauthorized().body("API key invalid")),
                Err(e) => return Err(ErrorInternalServerError(format!("{}", e))),
            }

            let mut image_url: String;
            loop{
                image_url = get_mem_url().await?;

                let token_to_find = key_value;

                let data_to_check = &image_url;

                let mut filter = doc! {
                    "issued_memes": {
                        "$elemMatch": { "$eq": &data_to_check }
                    }
                };

                filter.insert("token", Bson::String(token_to_find.to_string()));
                let options = FindOneOptions::builder().build();
                let result = collection.find_one(filter, options).await;

                match result {
                    Ok(Some(_)) => continue,
                    Ok(None) => break,
                    Err(e) => ErrorInternalServerError(format!("{}", e)),
                };
            }

            collection
                .update_one(
                    doc! { "token": key_value },
                    doc! { "$addToSet": { "issued_memes": &image_url } },
                    None,
                )
                .await
                .map_err(|e| ErrorInternalServerError(format!("{}", e)))?;


            let image_bytes = reqwest::get(&image_url)
                .await
                .map_err(|e| ErrorInternalServerError(format!("{}", e)))?
                .bytes()
                .await
                .map_err(|e| ErrorInternalServerError(format!("{}", e)))?
                .to_vec();

            Ok(HttpResponse::Ok().content_type("image/jpeg").body(image_bytes))
        }
        None => {
            Ok(HttpResponse::Unauthorized().body("Missing API key"))
        }
    }
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
