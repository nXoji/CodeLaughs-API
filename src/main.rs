use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod api;

// "/ping", to check the functionality
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_owned());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let addr = format!("{}:{}", host, port);

    println!("API is available at <http://{addr}/api/get_meme>");

    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .configure(api::init_routes)
    })
    .bind(addr)?
    .run()
    .await
}

