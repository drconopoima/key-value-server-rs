#[actix_rt::test]
async fn healthcheck_endpoint() {
    // Arrange
    launch_http_server();
    // Act
    // Client library makes HTTP requests against server
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/healthcheck")
        .send()
        .await
        .expect("Failed GET request to http://127.0.0.1:8080/healthcheck");
    // Assert
    // Status 200 OK
    assert!(response.status().is_success());
    // Empty Body
    assert_eq!(Some(0), response.content_length());
}

// Launch an instance for our HTTP server in the background
fn launch_http_server() {
    let server = key_value_server_rs::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
