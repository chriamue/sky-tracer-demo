use crate::services::FlightService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use sky_tracer::protocol::flights::{
    CreateFlightRequest, FlightPositionResponse, FlightResponse, ListFlightsRequest,
};
use sky_tracer::protocol::{FLIGHTS_API_PATH, FLIGHTS_POSITION_API_PATH};
use tracing::{debug, error, info, instrument, warn};

// Custom error type for our API
#[derive(Debug)]
pub enum ApiError {
    FlightCreationError(String),
    NotFound,
    ParseError(String),
    ServiceError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::FlightCreationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Flight not found".to_string()),
            ApiError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::ServiceError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

/// Create a new flight
#[utoipa::path(
    post,
    path = FLIGHTS_API_PATH,
    request_body = CreateFlightRequest,
    responses(
        (status = 201, description = "Flight created successfully", body = FlightResponse),
        (status = 400, description = "Invalid flight data provided"),
        (status = 500, description = "Internal server error")
    ),
    tag = "flights"
)]
#[instrument(skip(flight_service, request), fields(
    aircraft = %request.aircraft_number,
    departure = %request.departure,
    arrival = %request.arrival
))]
pub async fn create_flight(
    State(flight_service): State<FlightService>,
    Json(request): Json<CreateFlightRequest>,
) -> Result<(StatusCode, Json<FlightResponse>), ApiError> {
    debug!("Creating new flight");

    match flight_service.create_flight(request).await {
        Ok(flight) => {
            info!(
                flight_number = %flight.flight_number,
                "Flight created successfully"
            );

            let response = FlightResponse {
                flight_number: flight.flight_number,
                aircraft_number: flight.aircraft_number,
                departure: flight.departure,
                arrival: flight.arrival,
                departure_time: flight.departure_time,
                arrival_time: flight.arrival_time,
            };

            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            error!(error = %e, "Failed to create flight");
            Err(ApiError::FlightCreationError(e))
        }
    }
}

/// List flights
#[utoipa::path(
    get,
    path = FLIGHTS_API_PATH,
    params(
        ListFlightsRequest
    ),
    responses(
        (status = 200, description = "List of flights", body = Vec<FlightResponse>),
        (status = 400, description = "Invalid query parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "flights"
)]
#[instrument(skip(flight_service), fields(
    departure = ?params.departure,
    arrival = ?params.arrival,
    date = ?params.date
))]
pub async fn list_flights(
    State(flight_service): State<FlightService>,
    Query(params): Query<ListFlightsRequest>,
) -> Result<Json<Vec<FlightResponse>>, ApiError> {
    debug!("Listing flights with filters");

    let date = if let Some(date_str) = params.date {
        match DateTime::parse_from_rfc3339(&date_str) {
            Ok(dt) => {
                debug!(parsed_date = %dt, "Parsed date filter");
                Some(dt.with_timezone(&Utc))
            }
            Err(e) => {
                error!(error = %e, date_str = %date_str, "Failed to parse date");
                return Err(ApiError::ParseError(format!("Invalid date format: {}", e)));
            }
        }
    } else {
        None
    };

    let flights = flight_service
        .list_flights(params.departure, params.arrival, date)
        .await;

    info!(
        flights_count = flights.len(),
        "Retrieved flights matching criteria"
    );

    let response: Vec<FlightResponse> = flights
        .into_iter()
        .map(|flight| FlightResponse {
            flight_number: flight.flight_number,
            aircraft_number: flight.aircraft_number,
            departure: flight.departure,
            arrival: flight.arrival,
            departure_time: flight.departure_time,
            arrival_time: flight.arrival_time,
        })
        .collect();

    Ok(Json(response))
}

/// Get flight position
#[utoipa::path(
    get,
    path = FLIGHTS_POSITION_API_PATH,
    responses(
        (status = 200, description = "Flight position retrieved", body = FlightPositionResponse),
        (status = 404, description = "Flight not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("flight_number" = String, Path, description = "Flight number")
    ),
    tag = "flights"
)]
#[instrument(skip(flight_service), fields(flight_number = %flight_number))]
pub async fn get_flight_position(
    State(flight_service): State<FlightService>,
    Path(flight_number): Path<String>,
) -> Result<Json<FlightPositionResponse>, ApiError> {
    debug!("Fetching flight position for flight {}", flight_number);

    let flight = match flight_service.get_flight(&flight_number).await {
        Some(f) => {
            debug!(
                departure = %f.departure,
                arrival = %f.arrival,
                departure_time = %f.departure_time,
                "Found flight details"
            );
            f
        }
        None => {
            warn!("Flight not found: {}", flight_number);
            return Err(ApiError::NotFound);
        }
    };

    match flight_service.calculate_flight_position(&flight).await {
        Ok((latitude, longitude, timestamp)) => {
            info!(
                flight_number = %flight_number,
                latitude = latitude,
                longitude = longitude,
                timestamp = %timestamp,
                "Successfully retrieved flight position"
            );

            Ok(Json(FlightPositionResponse {
                flight_number: flight.flight_number,
                latitude,
                longitude,
                timestamp,
            }))
        }
        Err(e) => {
            error!(
                flight_number = %flight_number,
                error = %e,
                "Failed to calculate flight position"
            );
            Err(ApiError::ServiceError(format!(
                "Failed to calculate position: {}",
                e
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_create_flight_route() {
        let flight_service = FlightService::new();
        let request = CreateFlightRequest {
            aircraft_number: "D-ABCD".to_string(),
            departure: "FRA".to_string(),
            arrival: "LIS".to_string(),
            departure_time: Utc::now(),
            arrival_time: None,
        };

        let result = create_flight(State(flight_service), Json(request)).await;
        assert!(result.is_ok());

        let (status, response) = result.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(response.aircraft_number, "D-ABCD");
    }

    #[tokio::test]
    async fn test_list_flights_route() {
        let flight_service = FlightService::new();
        let params = ListFlightsRequest {
            departure: None,
            arrival: None,
            date: None,
        };

        let result = list_flights(State(flight_service), Query(params)).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.is_empty()); // No flights created yet
    }

    #[test]
    fn test_api_path_constants() {
        // Verify we're using the correct API paths
        assert_eq!(FLIGHTS_API_PATH, "/api/v1/flights");
        assert_eq!(
            FLIGHTS_POSITION_API_PATH,
            "/api/v1/flights/{flight_number}/position"
        );
    }
}
