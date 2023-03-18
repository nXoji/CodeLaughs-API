use actix_web::web;

mod handlers;
mod parse_data;

// Initialize the api routes, all the routes are under `/api`
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/get_meme", web::get().to(handlers::get_image))
    );
}
