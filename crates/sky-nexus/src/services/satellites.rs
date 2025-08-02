use reqwest::Client;
use sky_tracer::protocol::{
    SATELLITES_API_PATH, SATELLITES_POSITION_API_PATH, SATELLITES_STATUS_API_PATH,
    satellite::{
        CalculatePositionRequest, CalculatePositionResponse, CreateSatelliteRequest,
        SatelliteResponse, UpdateSatelliteStatusRequest,
    },
};
use std::env;
use thiserror::Error;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum SatelliteServiceError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Satellite not found: {0}")]
    NotFound(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

fn get_satellite_service_base_url() -> String {
    env::var("SATELLITE_SERVICE_BASE_URL").unwrap_or_else(|_| "http://localhost:3002".to_string())
}

#[instrument]
pub async fn fetch_satellites() -> Result<Vec<SatelliteResponse>, SatelliteServiceError> {
    let client = Client::new();
    let base_url = get_satellite_service_base_url();
    let url = format!("{}{}", base_url, SATELLITES_API_PATH);

    info!("Fetching satellites from: {}", url);

    let resp = client.get(&url).send().await?;
    if resp.status().is_success() {
        let satellites = resp.json::<Vec<SatelliteResponse>>().await?;
        info!("Successfully fetched {} satellites", satellites.len());
        Ok(satellites)
    } else {
        error!("Failed to fetch satellites: {}", resp.status());
        Err(SatelliteServiceError::Network(
            resp.error_for_status().unwrap_err(),
        ))
    }
}

#[instrument]
pub async fn create_satellite(
    req: CreateSatelliteRequest,
) -> Result<SatelliteResponse, SatelliteServiceError> {
    let client = Client::new();
    let base_url = get_satellite_service_base_url();
    let url = format!("{}{}", base_url, SATELLITES_API_PATH);

    info!("Creating satellite at: {}", url);

    let resp = client.post(&url).json(&req).send().await?;
    if resp.status().is_success() {
        let satellite = resp.json::<SatelliteResponse>().await?;
        info!("Successfully created satellite: {}", satellite.name);
        Ok(satellite)
    } else {
        error!("Failed to create satellite: {}", resp.status());
        Err(SatelliteServiceError::Network(
            resp.error_for_status().unwrap_err(),
        ))
    }
}

#[instrument]
pub async fn update_satellite_status(
    id: Uuid,
    req: UpdateSatelliteStatusRequest,
) -> Result<SatelliteResponse, SatelliteServiceError> {
    let client = Client::new();
    let base_url = get_satellite_service_base_url();
    let url = format!(
        "{}{}",
        base_url,
        SATELLITES_STATUS_API_PATH.replace("{id}", &id.to_string())
    );

    info!("Updating satellite status at: {}", url);

    let resp = client.put(&url).json(&req).send().await?;
    if resp.status().is_success() {
        let satellite = resp.json::<SatelliteResponse>().await?;
        info!("Successfully updated satellite: {}", satellite.name);
        Ok(satellite)
    } else {
        error!("Failed to update satellite status: {}", resp.status());
        Err(SatelliteServiceError::Network(
            resp.error_for_status().unwrap_err(),
        ))
    }
}

#[instrument]
pub async fn calculate_position(
    req: CalculatePositionRequest,
) -> Result<CalculatePositionResponse, SatelliteServiceError> {
    let client = Client::new();
    let base_url = get_satellite_service_base_url();
    let url = format!("{}{}", base_url, SATELLITES_POSITION_API_PATH);

    info!("Calculating position at: {}", url);

    let resp = client.post(&url).json(&req).send().await?;
    if resp.status().is_success() {
        let response = resp.json::<CalculatePositionResponse>().await?;
        info!(
            "Successfully calculated {} positions",
            response.positions.len()
        );
        Ok(response)
    } else {
        error!("Failed to calculate position: {}", resp.status());
        Err(SatelliteServiceError::Network(
            resp.error_for_status().unwrap_err(),
        ))
    }
}
