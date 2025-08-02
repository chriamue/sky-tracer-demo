use reqwest::Client;
use sky_tracer::protocol::{
    BABEL_AIRPORT_API_PATH, BABEL_POSITION_API_PATH,
    flights::{FlightPositionResponse, FlightResponse},
};
use std::env;
use thiserror::Error;
use tracing::{error, info, instrument};

#[derive(Error, Debug)]
pub enum BabelServiceError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("No future flights found for airport: {0}")]
    NoFutureFlights(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

fn get_babel_service_base_url() -> String {
    env::var("BABEL_SERVICE_BASE_URL").unwrap_or_else(|_| "http://localhost:3003".to_string())
}

#[instrument]
pub async fn fetch_flights_by_airport(
    airport_code: &str,
) -> Result<Vec<FlightResponse>, BabelServiceError> {
    info!("Fetching flights for airport: {}", airport_code);
    let client = Client::new();
    let base_url = get_babel_service_base_url();
    let url = format!(
        "{}{}",
        base_url,
        BABEL_AIRPORT_API_PATH.replace("{airport_code}", airport_code)
    );

    info!(url = %url, "Making request to fetch flights by airport");
    let resp = client.get(&url).send().await?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            let flights = resp.json::<Vec<FlightResponse>>().await.map_err(|e| {
                error!("Failed to parse flights response: {}", e);
                BabelServiceError::ParseError(format!("JSON parse error: {}", e))
            })?;

            info!(count = flights.len(), airport_code = %airport_code, "Successfully fetched flights");
            Ok(flights)
        }
        reqwest::StatusCode::NOT_FOUND => {
            let error_text = resp
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Check if it's specifically "no future flights" vs "airport not found"
            if error_text.contains("No future flights") || error_text.contains("NO_FUTURE_FLIGHTS")
            {
                Err(BabelServiceError::NoFutureFlights(airport_code.to_string()))
            } else {
                Err(BabelServiceError::NotFound(format!(
                    "Airport {} not found or no data available",
                    airport_code
                )))
            }
        }
        status => {
            error!(status = %status, airport_code = %airport_code, "Failed to fetch flights");
            Err(BabelServiceError::Network(reqwest::Error::from(
                resp.error_for_status().unwrap_err(),
            )))
        }
    }
}

#[instrument]
pub async fn fetch_flight_position(
    flight_number: &str,
) -> Result<FlightPositionResponse, BabelServiceError> {
    info!("Fetching position for flight: {}", flight_number);
    let client = Client::new();
    let base_url = get_babel_service_base_url();
    let url = format!(
        "{}{}",
        base_url,
        BABEL_POSITION_API_PATH.replace("{flight_number}", flight_number)
    );

    info!(url = %url, "Making request to fetch flight position");
    let resp = client.get(&url).send().await?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            let position = resp.json::<FlightPositionResponse>().await.map_err(|e| {
                error!("Failed to parse flight position response: {}", e);
                BabelServiceError::ParseError(format!("JSON parse error: {}", e))
            })?;

            info!(
                flight_number = %flight_number,
                latitude = position.latitude,
                longitude = position.longitude,
                "Successfully fetched flight position"
            );
            Ok(position)
        }
        reqwest::StatusCode::NOT_FOUND => Err(BabelServiceError::NotFound(format!(
            "Flight {} not found or not currently in flight",
            flight_number
        ))),
        status => {
            error!(status = %status, flight_number = %flight_number, "Failed to fetch flight position");
            Err(BabelServiceError::Network(reqwest::Error::from(
                resp.error_for_status().unwrap_err(),
            )))
        }
    }
}
