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
use tracing::{debug, error, info, instrument, warn};

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
    path = "/api/flights/",
    request_body = CreateFlightRequest,
    responses(
        (status = 200, description = "Flight created successfully", body = FlightResponse),
        (status = 400, description = "Invalid flight data provided"),
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
) -> Result<Json<FlightResponse>, ApiError> {
    debug!("Creating new flight");

    flight_service
        .create_flight(request)
        .await
        .map(|flight| {
            info!(
                flight_number = %flight.flight_number,
                "Flight created successfully"
            );
            Json(FlightResponse {
                flight_number: flight.flight_number,
                aircraft_number: flight.aircraft_number,
                departure: flight.departure,
                arrival: flight.arrival,
                departure_time: flight.departure_time,
                arrival_time: flight.arrival_time,
            })
        })
        .map_err(|e| {
            error!(error = %e, "Failed to create flight");
            ApiError::FlightCreationError(e)
        })
}

/// List flights
///
/// Retrieves a list of flights, optionally filtered by departure, arrival, and date.
#[utoipa::path(
    get,
    path = "/api/flights/",
    params(
        ListFlightsRequest
    ),
    responses(
        (status = 200, description = "List of flights", body = Vec<FlightResponse>),
        (status = 400, description = "Invalid query parameters"),
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
                error!(error = %e, "Failed to parse date");
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

    // Create HTTP client with tracing middleware
    let client: ClientWithMiddleware = ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware::default())
        .build();

    let orbital_beacon_url = std::env::var("ORBITAL_BEACON_URL").unwrap_or_else(|_| {
        debug!("ORBITAL_BEACON_URL not set, using default");
        "http://orbital-beacon:3002".to_string()
    });

    debug!(url = %orbital_beacon_url, "Using orbital beacon URL");

    let arrival_time = flight.arrival_time.unwrap_or_else(|| {
        let calculated_time = flight.departure_time + chrono::Duration::hours(2);
        debug!(
            departure_time = %flight.departure_time,
            calculated_arrival = %calculated_time,
            "Calculated default arrival time"
        );
        calculated_time
    });

    let position_request = sky_tracer::protocol::satellite::CalculatePositionRequest {
        departure: flight.departure.clone(),
        arrival: flight.arrival.clone(),
        departure_time: flight.departure_time,
        arrival_time,
        current_time: Some(chrono::Utc::now()),
    };

    debug!(
        departure = %position_request.departure,
        arrival = %position_request.arrival,
        departure_time = %position_request.departure_time,
        arrival_time = %position_request.arrival_time,
        "Preparing position request"
    );

    // Convert to JSON string first
    let json_body = match serde_json::to_string(&position_request) {
        Ok(body) => {
            debug!(request_body = %body, "Serialized request body");
            body
        }
        Err(e) => {
            error!(error = %e, "Failed to serialize position request");
            return Err(ApiError::ParseError(format!(
                "Failed to serialize request: {}",
                e
            )));
        }
    };

    debug!("Sending position request to orbital beacon");

    match client
        .post(&format!("{}/api/position", orbital_beacon_url))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            debug!(status = %status, "Received response from orbital beacon");

            if response.status().is_success() {
                match response
                    .json::<sky_tracer::protocol::satellite::CalculatePositionResponse>()
                    .await
                {
                    Ok(position_data) => {
                        debug!(
                            positions_count = position_data.positions.len(),
                            "Received position data"
                        );

                        match position_data.positions.first() {
                            Some(position) => {
                                info!(
                                    flight_number = %flight_number,
                                    latitude = position.latitude,
                                    longitude = position.longitude,
                                    timestamp = %position.timestamp,
                                    "Successfully calculated flight position"
                                );

                                Ok(Json(FlightPositionResponse {
                                    flight_number: flight.flight_number,
                                    latitude: position.latitude,
                                    longitude: position.longitude,
                                    timestamp: position.timestamp,
                                }))
                            }
                            None => {
                                warn!("No position data available in response");
                                Err(ApiError::FlightCreationError(
                                    "No position data available".to_string(),
                                ))
                            }
                        }
                    }
                    Err(e) => {
                        error!(error = %e, "Failed to parse position response");
                        Err(ApiError::ParseError(e.to_string()))
                    }
                }
            } else {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                error!(
                    status = %status,
                    error = %error_text,
                    "Failed to calculate position"
                );
                Err(ApiError::FlightCreationError(format!(
                    "Failed to calculate position: {}",
                    error_text
                )))
            }
        }
        Err(e) => {
            error!(error = %e, "Failed to connect to orbital beacon");
            Err(ApiError::FlightCreationError(format!(
                "Failed to connect to orbital beacon: {}",
                e
            )))
        }
    }
}
