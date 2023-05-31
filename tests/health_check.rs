#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();

    // Act
    let client = reqwest::Client::new();

    let res = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

async fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
