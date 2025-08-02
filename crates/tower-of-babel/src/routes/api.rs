use crate::services::{BabelService, BabelServiceError};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sky_tracer::protocol::{
    flights::ErrorResponse, BABEL_AIRPORT_API_PATH, BABEL_POSITION_API_PATH,
};
use tracing::{error, instrument};

/// List flights by airport
#[utoipa::path(
    get,
    path = BABEL_AIRPORT_API_PATH,
    params(
        ("airport_code" = String, Path, description = "Airport IATA/ICAO code")
    ),
    responses(
        (status = 200, description = "List of future flights", body = Vec<sky_tracer::protocol::flights::FlightResponse>),
        (status = 404, description = "No flights found", body = ErrorResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "flights"
)]
#[instrument(skip(service), fields(airport_code = %airport_code))]
pub async fn get_flights_by_airport(
    Path(airport_code): Path<String>,
    State(service): State<BabelService>,
) -> impl IntoResponse {
    match service.list_flights_by_airport(&airport_code).await {
        Ok(flights) => (StatusCode::OK, Json(flights)).into_response(),
        Err(BabelServiceError::NotFound(msg)) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: msg,
                code: "NOT_FOUND".to_string(),
            }),
        )
            .into_response(),
        Err(BabelServiceError::NoFutureFlights(airport)) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("No future flights found for airport {}", airport),
                code: "NO_FUTURE_FLIGHTS".to_string(),
            }),
        )
            .into_response(),
        Err(e) => {
            error!(error = %e, airport_code = %airport_code, "Failed to get flights");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                    code: "INTERNAL_ERROR".to_string(),
                }),
            )
                .into_response()
        }
    }
}

/// Get flight position
#[utoipa::path(
    get,
    path = BABEL_POSITION_API_PATH,
    params(
        ("flight_number" = String, Path, description = "Flight number")
    ),
    responses(
        (status = 200, description = "Flight position retrieved", body = sky_tracer::protocol::flights::FlightPositionResponse),
        (status = 404, description = "Flight not found", body = ErrorResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "flights"
)]
#[instrument(skip(service), fields(flight_number = %flight_number))]
pub async fn get_flight_position(
    Path(flight_number): Path<String>,
    State(service): State<BabelService>,
) -> impl IntoResponse {
    // ... implementation stays the same
    match service.get_flight_position(&flight_number).await {
        Ok(position) => (StatusCode::OK, Json(position)).into_response(),
        Err(BabelServiceError::NotFound(msg)) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: msg,
                code: "NOT_FOUND".to_string(),
            }),
        )
            .into_response(),
        Err(e) => {
            error!(error = %e, flight_number = %flight_number, "Failed to get flight position");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                    code: "INTERNAL_ERROR".to_string(),
                }),
            )
                .into_response()
        }
    }
}
