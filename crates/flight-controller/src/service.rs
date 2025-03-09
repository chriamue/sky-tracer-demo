use crate::flight_service::FlightService;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use sky_tracer::protocol::flights::{CreateFlightRequest, FlightResponse, ListFlightsRequest};

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
