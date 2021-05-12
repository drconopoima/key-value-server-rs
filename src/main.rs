use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/health", web::get().to(health)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
