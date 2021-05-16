use key_value_server_rs::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Raise io::Error if address failed to bind
    // Otherwise call .await on Server
    run()?.await
}
