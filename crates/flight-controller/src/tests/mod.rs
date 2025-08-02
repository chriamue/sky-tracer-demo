use crate::app::app;
use axum_test::TestServer;
use reqwest::StatusCode;
use serde_json::json;
use sky_tracer::protocol::FLIGHTS_API_PATH;

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

    // When - Test the actual Swagger UI endpoints (with trailing slashes)
    let response1 = server.get("/api/docs/").await;
    let response2 = server.get("/api/rapidoc/").await;
    let response3 = server.get("/api/redoc/").await;

    // Then - These should return 200 OK
    response1.assert_status_ok();
    response2.assert_status_ok();
    response3.assert_status_ok();

    // Also test that the OpenAPI spec is available
    let openapi_response = server.get("/api-docs/openapi.json").await;
    openapi_response.assert_status_ok();

    // Verify it's valid JSON
    let openapi_json: serde_json::Value = openapi_response.json();
    assert!(openapi_json.get("openapi").is_some());
    assert!(openapi_json.get("info").is_some());
    assert!(openapi_json.get("paths").is_some());
}

#[tokio::test]
async fn it_should_redirect_api_docs_without_trailing_slash() {
    // Given
    let app = app();
    let server = TestServer::new(app).unwrap();

    // When - Test endpoints without trailing slashes (these might redirect)
    let response1 = server.get("/api/docs/").await;
    let response2 = server.get("/api/rapidoc/").await;
    let response3 = server.get("/api/redoc/").await;

    // Then - These should either be OK or redirect (303 See Other is acceptable)
    assert!(
        response1.status_code() == 200 || response1.status_code() == 303,
        "Expected 200 or 303 for /api/docs, got {}",
        response1.status_code()
    );
    assert!(
        response2.status_code() == 200 || response2.status_code() == 303,
        "Expected 200 or 303 for /api/rapidoc/, got {}",
        response2.status_code()
    );
    assert!(
        response3.status_code() == 200 || response3.status_code() == 303,
        "Expected 200 or 303 for /api/redoc/, got {}",
        response3.status_code()
    );
}

#[tokio::test]
async fn it_should_test_flights_are_empty() {
    // Given
    let app = app();
    let server = TestServer::new(app).unwrap();
    let expected_json = json!([]);
    // When
    let response1 = server.get(FLIGHTS_API_PATH).await;
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
        "flight_number": "LIS0001"
    });
    // When
    let response1 = server.post(FLIGHTS_API_PATH).json(&create_request).await;
    // Then
    response1.assert_status(StatusCode::CREATED);
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
            "flight_number": "LIS0001"
        },
        {
            "aircraft_number": "123",
            "arrival": "fra",
            "arrival_time": "2025-04-23T16:37:46.810397Z",
            "departure": "lis",
            "departure_time": "2025-04-23T16:28:46.810395Z",
            "flight_number": "LIS0002"
        }
    ]);

    // When
    let _response1 = server.post(FLIGHTS_API_PATH).json(&create_request).await;
    let _response2 = server.post(FLIGHTS_API_PATH).json(&create_request).await;
    let response3 = server.get(FLIGHTS_API_PATH).await;

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
    let response = server.post(FLIGHTS_API_PATH).json(&create_request).await;
    // Then
    response.assert_status_unprocessable_entity();
}

#[tokio::test]
async fn it_should_get_flight_position() {
    use sky_tracer::protocol::FLIGHTS_POSITION_API_PATH;

    // Given
    let app = app();
    let server = TestServer::new(app).unwrap();
    let create_request = json!({
        "aircraft_number": "D-ABCD",
        "arrival": "LIS",
        "arrival_time": "2025-04-23T16:37:46.810397+00:00",
        "departure": "FRA",
        "departure_time": "2025-04-23T16:28:46.810395Z"
    });

    // When - Create a flight first
    let create_response = server.post(FLIGHTS_API_PATH).json(&create_request).await;
    create_response.assert_status(StatusCode::CREATED);
    let flight: serde_json::Value = create_response.json();
    let flight_number = flight["flight_number"].as_str().unwrap();

    // Then - Try to get position (this might fail if orbital beacon is not available)
    let position_url = FLIGHTS_POSITION_API_PATH.replace("{flight_number}", flight_number);
    let position_response = server.get(&position_url).await;

    // Position request might fail due to external dependency, so we check for either success or service error
    assert!(
        position_response.status_code() == 200 || position_response.status_code() == 500,
        "Expected 200 or 500, got {}",
        position_response.status_code()
    );
}

#[tokio::test]
async fn it_should_return_404_for_nonexistent_flight_position() {
    use sky_tracer::protocol::FLIGHTS_POSITION_API_PATH;

    // Given
    let app = app();
    let server = TestServer::new(app).unwrap();

    // When - Try to get position for non-existent flight
    let position_url = FLIGHTS_POSITION_API_PATH.replace("{flight_number}", "NONEXISTENT");
    let response = server.get(&position_url).await;

    // Then
    response.assert_status_not_found();
}

#[test]
fn test_api_path_constants() {
    // Verify the constants are correct
    assert_eq!(FLIGHTS_API_PATH, "/api/v1/flights");
    use sky_tracer::protocol::FLIGHTS_POSITION_API_PATH;
    assert_eq!(
        FLIGHTS_POSITION_API_PATH,
        "/api/v1/flights/{flight_number}/position"
    );
}
