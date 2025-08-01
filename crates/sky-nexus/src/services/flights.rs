use reqwest::Client;
use sky_tracer::model::flight::Flight;
use sky_tracer::protocol::flights::FlightResponse;
use std::env;
use tracing::{debug, error, info};

#[derive(Debug)]
pub enum FlightServiceError {
    Network(reqwest::Error),
    NotFound(String),
    ParseError(String),
}

impl From<reqwest::Error> for FlightServiceError {
    fn from(err: reqwest::Error) -> Self {
        FlightServiceError::Network(err)
    }
}

impl std::fmt::Display for FlightServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlightServiceError::Network(e) => write!(f, "Network error: {}", e),
            FlightServiceError::NotFound(msg) => write!(f, "Not found: {}", msg),
            FlightServiceError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for FlightServiceError {}

fn get_flight_service_url() -> String {
    env::var("FLIGHT_SERVICE_URL").unwrap_or_else(|_| "http://localhost:3001/api".to_string())
}

pub async fn fetch_flights() -> Result<Vec<Flight>, FlightServiceError> {
    let client = Client::new();
    let url = format!("{}/flights/", get_flight_service_url());

    info!("Fetching flights from: {}", url);

    let resp = client.get(&url).send().await?;
    let status = resp.status();

    debug!("Response status: {}", status);

    if !status.is_success() {
        error!("Flight service returned error status: {}", status);
        return Err(FlightServiceError::Network(
            resp.error_for_status().unwrap_err(),
        ));
    }

    // Get response text first to debug
    let response_text = resp.text().await?;
    debug!("Response body: {}", response_text);

    // If empty response, return empty vec
    if response_text.trim().is_empty() {
        info!("Empty response from flight service, returning empty flight list");
        return Ok(vec![]);
    }

    // Try to parse as JSON
    let flight_responses: Vec<FlightResponse> =
        serde_json::from_str(&response_text).map_err(|e| {
            error!("Failed to parse flight response as JSON: {}", e);
            error!("Response was: {}", response_text);
            FlightServiceError::ParseError(format!(
                "JSON parse error: {}, response: {}",
                e, response_text
            ))
        })?;

    // Convert to Flight models
    let flights: Vec<Flight> = flight_responses
        .into_iter()
        .map(|fr| Flight {
            flight_number: fr.flight_number,
            aircraft_number: fr.aircraft_number,
            departure: fr.departure,
            arrival: fr.arrival,
            departure_time: fr.departure_time,
            arrival_time: fr.arrival_time,
        })
        .collect();

    info!("Successfully fetched {} flights", flights.len());
    Ok(flights)
}

pub async fn fetch_flight_by_number(flight_number: &str) -> Result<Flight, FlightServiceError> {
    info!("Fetching flight by number: {}", flight_number);

    // Since Flight Controller doesn't have individual flight lookup,
    // fetch all flights and filter
    let flights = fetch_flights().await?;

    flights
        .into_iter()
        .find(|f| f.flight_number == flight_number)
        .ok_or_else(|| {
            error!("Flight not found: {}", flight_number);
            FlightServiceError::NotFound(format!("Flight {} not found", flight_number))
        })
}

pub async fn create_flight(flight: Flight) -> Result<Flight, FlightServiceError> {
    use sky_tracer::protocol::flights::CreateFlightRequest;

    let client = Client::new();
    let url = format!("{}/flights/", get_flight_service_url());

    info!("Creating flight at: {}", url);

    // Convert Flight to CreateFlightRequest
    let create_request = CreateFlightRequest {
        aircraft_number: flight.aircraft_number,
        departure: flight.departure,
        arrival: flight.arrival,
        departure_time: flight.departure_time,
        arrival_time: flight.arrival_time,
    };

    debug!("Create request: {:?}", create_request);

    let resp = client.post(&url).json(&create_request).send().await?;
    let status = resp.status();

    debug!("Create response status: {}", status);

    if !status.is_success() {
        error!("Flight creation failed with status: {}", status);
        return Err(FlightServiceError::Network(
            resp.error_for_status().unwrap_err(),
        ));
    }

    let response_text = resp.text().await?;
    debug!("Create response body: {}", response_text);

    let flight_response: FlightResponse = serde_json::from_str(&response_text).map_err(|e| {
        error!("Failed to parse create flight response: {}", e);
        error!("Response was: {}", response_text);
        FlightServiceError::ParseError(format!(
            "JSON parse error: {}, response: {}",
            e, response_text
        ))
    })?;

    // Convert back to Flight
    let created_flight = Flight {
        flight_number: flight_response.flight_number,
        aircraft_number: flight_response.aircraft_number,
        departure: flight_response.departure,
        arrival: flight_response.arrival,
        departure_time: flight_response.departure_time,
        arrival_time: flight_response.arrival_time,
    };

    info!(
        "Successfully created flight: {}",
        created_flight.flight_number
    );
    Ok(created_flight)
}
