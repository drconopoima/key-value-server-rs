use key_value_server_rs::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
