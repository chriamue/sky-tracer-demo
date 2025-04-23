use crate::app::app;
use axum_test::TestServer;

#[tokio::test]
async fn it_should_response_on_root_with_html_page() {
    // Given
    let app = app();
    let server = TestServer::new(app).unwrap();
    // When
    let response = server.get("/").await;
    // Then
    response.assert_status_ok();
    response.assert_text_contains("<!DOCTYPE html>");
}

#[tokio::test]
async fn it_should_response_200_for_api_docs() {
    // Given
    let app = app();
    let server = TestServer::new(app).unwrap();
    // When
    let response1 = server.get("/api/docs/").await;
    let response2 = server.get("/api/rapidoc/").await;
    let response3 = server.get("/api/redoc/").await;
    // Then
    response1.assert_status_ok();
    response2.assert_status_ok();
    response3.assert_status_ok();
}
