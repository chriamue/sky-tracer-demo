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
