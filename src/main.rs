use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use std::env;

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;

async fn is_alive() -> impl Responder {
    HttpResponse::Ok().body("born to be alive!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let port: u16 = match env::var("PORT") {
        Ok(p) => p.parse().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT,
    };

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(is_alive))
            .route("/status", web::get().to(is_alive))
    })
    .bind((DEFAULT_HOST, port))?
    .run()
    .await
}
