use actix_web::{web, App, HttpResponse, HttpServer, Responder};

const HOST: &str = "0.0.0.0";
const PORT: u16 = 8080;

async fn is_alive() -> impl Responder {
    HttpResponse::Ok().body("born to be alive!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(is_alive))
            .route("/status", web::get().to(is_alive))
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
