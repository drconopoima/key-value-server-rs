use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/healthcheck", web::get().to(healthcheck)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
