use crate::app::app;
use axum_test::TestServer;
use serde_json::json;

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

#[tokio::test]
async fn it_should_test_flights_are_empty() {
    // Given
    let app = app();
    let server = TestServer::new(app).unwrap();
    let expected_json = json!([]);
    // When
    let response1 = server.get("/api/flights/").await;
    // Then
    response1.assert_status_ok();
    response1.assert_json(&expected_json);
}

#[tokio::test]
async fn it_should_create_flight() {
    // Given
    let app = app();
    let server = TestServer::new(app).unwrap();
    let create_request = json!({
        "aircraft_number": "123",
        "arrival": "fra",
        "arrival_time": "2025-04-23T16:37:46.810397+00:00",
        "departure": "lis",
        "departure_time": "2025-04-23T16:28:46.810395Z"
    });
    let expected_response = json!({
        "aircraft_number": "123",
        "arrival": "fra",
        "arrival_time": "2025-04-23T16:37:46.810397Z",
        "departure": "lis",
        "departure_time": "2025-04-23T16:28:46.810395Z",
        "flight_number": "lis0001"
    });
    // When
    let response1 = server.post("/api/flights/").json(&create_request).await;
    // Then
    response1.assert_status_ok();
    response1.assert_json(&expected_response);
}

#[tokio::test]
async fn it_response_with_list_of_flights_with_multiple_entries() {
    use serde_json_assert::{assert_json_matches, CompareMode, Config};

    // Given
    let app = app();
    let server = TestServer::new(app).unwrap();
    let create_request = json!({
        "aircraft_number": "123",
        "arrival": "fra",
        "arrival_time": "2025-04-23T16:37:46.810397+00:00",
        "departure": "lis",
        "departure_time": "2025-04-23T16:28:46.810395Z"
    });

    let expected_response = json!([
        {
            "aircraft_number": "123",
            "arrival": "fra",
            "arrival_time": "2025-04-23T16:37:46.810397Z",
            "departure": "lis",
            "departure_time": "2025-04-23T16:28:46.810395Z",
            "flight_number": "lis0001"
        },
        {
            "aircraft_number": "123",
            "arrival": "fra",
            "arrival_time": "2025-04-23T16:37:46.810397Z",
            "departure": "lis",
            "departure_time": "2025-04-23T16:28:46.810395Z",
            "flight_number": "lis0002"
        }
    ]);

    // When
    let _response1 = server.post("/api/flights/").json(&create_request).await;
    let _response2 = server.post("/api/flights/").json(&create_request).await;
    let response3 = server.get("/api/flights/").await;

    // Then
    response3.assert_status_ok();
    let response_json: serde_json::Value = response3.json();

    let config = Config::new(CompareMode::Strict).consider_array_sorting(false);
    assert_json_matches!(&response_json, &expected_response, &config);
}

#[tokio::test]
async fn it_should_return_error_when_aircraft_number_is_missing() {
    // Given
    let app = app();
    let server = TestServer::new(app).unwrap();
    let create_request = json!({
        "arrival": "fra",
        "arrival_time": "2025-04-23T16:37:46.810397+00:00",
        "departure": "lis",
        "departure_time": "2025-04-23T16:28:46.810395Z"
    });
    // When
    let response = server.post("/api/flights/").json(&create_request).await;
    // Then
    response.assert_status_unprocessable_entity();
}
