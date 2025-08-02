use crate::services::flights::{
    FlightServiceError, create_flight, fetch_flight_by_number, fetch_flights,
};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use sky_tracer::model::flight::Flight;
use sky_tracer::protocol::flights::{CreateFlightRequest, FlightResponse};
use tracing::{error, info};

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_flights).post(post_flight))
        .route("/{flight_number}", get(get_flight))
}

#[utoipa::path(
    get,
    path = "/api/v1/flights",
    responses(
        (status = 200, description = "List all flights", body = [FlightResponse])
    ),
    tag = "Flights"
)]
pub async fn list_flights() -> impl IntoResponse {
    info!("Listing flights");

    match fetch_flights().await {
        Ok(flights) => {
            let responses = flights
                .into_iter()
                .map(|f| FlightResponse {
                    flight_number: f.flight_number,
                    aircraft_number: f.aircraft_number,
                    departure: f.departure,
                    arrival: f.arrival,
                    departure_time: f.departure_time,
                    arrival_time: f.arrival_time,
                })
                .collect::<Vec<_>>();

            info!("Successfully listed {} flights", responses.len());
            (StatusCode::OK, Json(responses)).into_response()
        }
        Err(e) => {
            error!("Failed to fetch flights: {}", e);
            let status = match e {
                FlightServiceError::NotFound(_) => StatusCode::NOT_FOUND,
                FlightServiceError::Network(_) => StatusCode::BAD_GATEWAY,
                FlightServiceError::ParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (
                status,
                Json(serde_json::json!({
                    "error": "Failed to fetch flights",
                    "details": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/flights",
    request_body = CreateFlightRequest,
    responses(
        (status = 201, description = "Flight created", body = FlightResponse)
    ),
    tag = "Flights"
)]
pub async fn post_flight(Json(request): Json<CreateFlightRequest>) -> impl IntoResponse {
    info!(
        "Creating flight: {} -> {}",
        request.departure, request.arrival
    );

    // Convert CreateFlightRequest to Flight model
    let flight = Flight {
        flight_number: format!("{}0001", request.departure), // Temporary
        aircraft_number: request.aircraft_number,
        departure: request.departure,
        arrival: request.arrival,
        departure_time: request.departure_time,
        arrival_time: request.arrival_time,
    };

    match create_flight(flight).await {
        Ok(created) => {
            let response = FlightResponse {
                flight_number: created.flight_number,
                aircraft_number: created.aircraft_number,
                departure: created.departure,
                arrival: created.arrival,
                departure_time: created.departure_time,
                arrival_time: created.arrival_time,
            };

            info!("Successfully created flight: {}", response.flight_number);
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            error!("Failed to create flight: {}", e);
            let status = match e {
                FlightServiceError::NotFound(_) => StatusCode::NOT_FOUND,
                FlightServiceError::Network(_) => StatusCode::BAD_GATEWAY,
                FlightServiceError::ParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (
                status,
                Json(serde_json::json!({
                    "error": "Failed to create flight",
                    "details": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/flights/{flight_number}",
    params(
        ("flight_number" = String, Path, description = "Flight number")
    ),
    responses(
        (status = 200, description = "Get flight by number", body = FlightResponse),
        (status = 404, description = "Flight not found")
    ),
    tag = "Flights"
)]
pub async fn get_flight(
    axum::extract::Path(flight_number): axum::extract::Path<String>,
) -> impl IntoResponse {
    info!("Getting flight: {}", flight_number);

    match fetch_flight_by_number(&flight_number).await {
        Ok(flight) => {
            let response = FlightResponse {
                flight_number: flight.flight_number,
                aircraft_number: flight.aircraft_number,
                departure: flight.departure,
                arrival: flight.arrival,
                departure_time: flight.departure_time,
                arrival_time: flight.arrival_time,
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(FlightServiceError::NotFound(_)) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Flight not found",
                "flight_number": flight_number
            })),
        )
            .into_response(),
        Err(e) => {
            error!("Failed to get flight {}: {}", flight_number, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get flight",
                    "details": e.to_string()
                })),
            )
                .into_response()
        }
    }
}
