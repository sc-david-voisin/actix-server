use actix_files as fs;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

use std::env;

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;

#[get("/status")]
async fn is_alive() -> impl Responder {
    HttpResponse::Ok().body("born to be alive!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = match env::var("PORT") {
        Ok(p) => p.parse().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT,
    };

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(is_alive)
            .service(fs::Files::new("/", "./templates").index_file("index.html"))
    })
    .bind((DEFAULT_HOST, port))?
    .run()
    .await
}
