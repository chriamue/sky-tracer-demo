use crate::flight_service::FlightService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_tracing::TracingMiddleware;
use sky_tracer::protocol::flights::{
    CreateFlightRequest, FlightPositionResponse, FlightResponse, ListFlightsRequest,
};

// Custom error type for our API
#[derive(Debug)]
pub enum ApiError {
    FlightCreationError(String),
    NotFound,
    ParseError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::FlightCreationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Flight not found".to_string()),
            ApiError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

/// Create a new flight
///
/// Creates a new flight with the provided details and returns the flight information.
#[utoipa::path(
    post,
    path = "/api/flights",
    request_body = CreateFlightRequest,
    responses(
        (status = 200, description = "Flight created successfully", body = FlightResponse),
        (status = 400, description = "Invalid flight data provided"),
    ),
    tag = "flights"
)]
pub async fn create_flight(
    State(flight_service): State<FlightService>,
    Json(request): Json<CreateFlightRequest>,
) -> Result<Json<FlightResponse>, ApiError> {
    flight_service
        .create_flight(request)
        .await
        .map(|flight| {
            Json(FlightResponse {
                flight_number: flight.flight_number,
                aircraft_number: flight.aircraft_number,
                departure: flight.departure,
                arrival: flight.arrival,
                departure_time: flight.departure_time,
                arrival_time: flight.arrival_time,
            })
        })
        .map_err(ApiError::FlightCreationError)
}

/// List flights
///
/// Retrieves a list of flights, optionally filtered by departure, arrival, and date.
#[utoipa::path(
    get,
    path = "/api/flights",
    params(
        ListFlightsRequest
    ),
    responses(
        (status = 200, description = "List of flights", body = Vec<FlightResponse>),
        (status = 400, description = "Invalid query parameters"),
    ),
    tag = "flights"
)]
pub async fn list_flights(
    State(flight_service): State<FlightService>,
    Query(params): Query<ListFlightsRequest>,
) -> Result<Json<Vec<FlightResponse>>, ApiError> {
    let date = if let Some(date_str) = params.date {
        Some(
            DateTime::parse_from_rfc3339(&date_str)
                .map_err(|e| ApiError::ParseError(format!("Invalid date format: {}", e)))?
                .with_timezone(&Utc),
        )
    } else {
        None
    };

    let flights = flight_service
        .list_flights(params.departure, params.arrival, date)
        .await;

    Ok(Json(
        flights
            .into_iter()
            .map(|flight| FlightResponse {
                flight_number: flight.flight_number,
                aircraft_number: flight.aircraft_number,
                departure: flight.departure,
                arrival: flight.arrival,
                departure_time: flight.departure_time,
                arrival_time: flight.arrival_time,
            })
            .collect(),
    ))
}

/// Get flight position
#[utoipa::path(
    get,
    path = "/api/flights/{flight_number}/position",
    responses(
        (status = 200, description = "Flight position retrieved", body = FlightPositionResponse),
        (status = 404, description = "Flight not found"),
    ),
    params(
        ("flight_number" = String, Path, description = "Flight number")
    ),
    tag = "flights"
)]
pub async fn get_flight_position(
    State(flight_service): State<FlightService>,
    Path(flight_number): Path<String>,
) -> Result<Json<FlightPositionResponse>, ApiError> {
    let flight = flight_service
        .get_flight(&flight_number)
        .await
        .ok_or(ApiError::NotFound)?;

    // Create HTTP client with tracing middleware
    let client: ClientWithMiddleware = ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware::default())
        .build();

    let orbital_beacon_url = std::env::var("ORBITAL_BEACON_URL")
        .unwrap_or_else(|_| "http://orbital-beacon:3002".to_string());

    let position_request = sky_tracer::protocol::satellite::CalculatePositionRequest {
        departure: flight.departure.clone(),
        arrival: flight.arrival.clone(),
        departure_time: flight.departure_time,
        arrival_time: flight
            .arrival_time
            .unwrap_or_else(|| flight.departure_time + chrono::Duration::hours(2)),
        current_time: None, // Use current time
    };

    // Convert to JSON string first
    let json_body = serde_json::to_string(&position_request)
        .map_err(|e| ApiError::ParseError(format!("Failed to serialize request: {}", e)))?;

    match client
        .post(&format!("{}/api/position", orbital_beacon_url))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let position_data = response
                    .json::<sky_tracer::protocol::satellite::CalculatePositionResponse>()
                    .await
                    .map_err(|e| ApiError::ParseError(e.to_string()))?;

                // Get the first position from the response
                let position = position_data.positions.first().ok_or_else(|| {
                    ApiError::FlightCreationError("No position data available".to_string())
                })?;

                Ok(Json(FlightPositionResponse {
                    flight_number: flight.flight_number,
                    latitude: position.latitude,
                    longitude: position.longitude,
                    timestamp: position.timestamp,
                }))
            } else {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(ApiError::FlightCreationError(format!(
                    "Failed to calculate position: {}",
                    error_text
                )))
            }
        }
        Err(e) => Err(ApiError::FlightCreationError(format!(
            "Failed to connect to orbital beacon: {}",
            e
        ))),
    }
}
