use reqwest::Client;
use sky_tracer::model::airport::Airport;
use sky_tracer::protocol::{
    AIRPORTS_API_PATH, AIRPORTS_SEARCH_API_PATH, airports::SearchAirportsResponse,
};
use std::env;
use thiserror::Error;
use tracing::{error, info, instrument};

#[derive(Error, Debug)]
pub enum AirportServiceError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Airport not found: {0}")]
    NotFound(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

fn get_airport_service_base_url() -> String {
    env::var("AIRPORT_SERVICE_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

#[instrument]
pub async fn fetch_airports() -> Result<Vec<Airport>, AirportServiceError> {
    info!("Fetching all airports");
    let client = Client::new();
    let base_url = get_airport_service_base_url();
    let url = format!("{}{}", base_url, AIRPORTS_API_PATH);

    info!(url = %url, "Making request to fetch airports");
    let resp = client.get(&url).send().await?;

    if resp.status().is_success() {
        let search_response = resp.json::<SearchAirportsResponse>().await?;
        let airports: Vec<Airport> = search_response
            .airports
            .into_iter()
            .map(|airport_response| Airport {
                id: airport_response.id,
                latitude: airport_response.position.latitude,
                longitude: airport_response.position.longitude,
                name: airport_response.name,
                code: airport_response.code,
            })
            .collect();

        info!(count = airports.len(), "Successfully fetched airports");
        Ok(airports)
    } else {
        error!(status = %resp.status(), "Failed to fetch airports");
        Err(AirportServiceError::Network(
            resp.error_for_status().unwrap_err(),
        ))
    }
}

#[instrument]
pub async fn fetch_airport_by_code(code: &str) -> Result<Airport, AirportServiceError> {
    info!(code = %code, "Fetching airport by code");
    let client = Client::new();
    let base_url = get_airport_service_base_url();
    let url = format!("{}{}?code={}", base_url, AIRPORTS_SEARCH_API_PATH, code);

    info!(url = %url, "Making request to search airport");
    let resp = client.get(&url).send().await?;

    if resp.status().is_success() {
        let search_response = resp.json::<SearchAirportsResponse>().await?;

        if let Some(airport_response) = search_response.airports.first() {
            let airport = Airport {
                id: airport_response.id,
                latitude: airport_response.position.latitude,
                longitude: airport_response.position.longitude,
                name: airport_response.name.clone(),
                code: airport_response.code.clone(),
            };

            info!(code = %code, name = %airport.name, "Successfully found airport");
            Ok(airport)
        } else {
            info!(code = %code, "Airport not found");
            Err(AirportServiceError::NotFound(format!(
                "Airport with code {} not found",
                code
            )))
        }
    } else {
        error!(status = %resp.status(), code = %code, "Failed to search airport");
        Err(AirportServiceError::Network(
            resp.error_for_status().unwrap_err(),
        ))
    }
}
