use crate::error::ApiError;
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use reqwest_middleware::ClientWithMiddleware;
use sky_tracer::protocol::flights::{ErrorResponse, FlightResponse};
use tracing::{debug, error, info, instrument, warn};

/// List flights by airport
///
/// Retrieves a list of flights departing from the specified airport with future arrivals.
#[utoipa::path(
    get,
    path = "/api/flights/{airport_code}",
    params(
        ("airport_code" = String, Path, description = "Airport IATA/ICAO code")
    ),
    responses(
        (status = 200, description = "List of future flights", body = Vec<FlightResponse>),
        (status = 404, description = "No flights found", body = ErrorResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
    ),
    tag = "flights"
)]
#[instrument(skip(flight_controller_url, client))]
pub async fn list_flights_by_airport(
    flight_controller_url: String,
    airport_code: String,
    client: ClientWithMiddleware,
) -> Result<Json<Vec<FlightResponse>>, ApiError> {
    let url = format!(
        "{}/api/flights?departure={}",
        flight_controller_url, airport_code
    );

    debug!(url = %url, airport = %airport_code, "Fetching flights");

    let response = client.get(&url).send().await.map_err(|e| {
        error!(error = %e, "Failed to fetch flights");
        ApiError::RequestError(format!("Failed to fetch flights: {}", e))
    })?;

    let status = response.status();
    debug!(status = %status, "Received response");

    if response.status().is_success() {
        let all_flights: Vec<FlightResponse> = response.json().await.map_err(|e| {
            error!(error = %e, "Failed to parse flights response");
            ApiError::ParseError(format!("Failed to parse flights: {}", e))
        })?;

        // Filter flights with future arrival times
        let now = Utc::now();
        let future_flights: Vec<FlightResponse> = all_flights
            .into_iter()
            .filter(|flight| {
                flight
                    .arrival_time
                    .map(|arrival| arrival > now)
                    .unwrap_or(true) // Include flights with no arrival time
            })
            .collect();

        info!(
            total_flights = future_flights.len(),
            airport = %airport_code,
            "Successfully retrieved future flights"
        );

        if future_flights.is_empty() {
            warn!(airport = %airport_code, "No future flights found");
            Err(ApiError::NotFound(format!(
                "No future flights found for airport {}",
                airport_code
            )))
        } else {
            Ok(Json(future_flights))
        }
    } else if response.status() == StatusCode::NOT_FOUND {
        warn!(airport = %airport_code, "No flights found");
        Err(ApiError::NotFound(format!(
            "No flights found for airport {}",
            airport_code
        )))
    } else {
        let error_message = response.text().await.unwrap_or_default();
        error!(
            status = %status,
            error = %error_message,
            airport = %airport_code,
            "Error from flight-controller"
        );
        Err(ApiError::RequestError(format!(
            "Flight controller returned an error: {}",
            error_message
        )))
    }
}
