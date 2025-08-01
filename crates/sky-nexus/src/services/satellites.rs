use reqwest::Client;
use sky_tracer::protocol::satellite::{
    CalculatePositionRequest, CalculatePositionResponse, CreateSatelliteRequest, SatelliteResponse,
    UpdateSatelliteStatusRequest,
};
use std::env;
use uuid::Uuid;

fn get_satellite_service_url() -> String {
    env::var("SATELLITE_SERVICE_URL").unwrap_or_else(|_| "http://localhost:3002/api".to_string())
}

pub async fn fetch_satellites() -> Result<Vec<SatelliteResponse>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/satellites", get_satellite_service_url());
    let resp = client.get(&url).send().await?;
    resp.json::<Vec<SatelliteResponse>>().await
}

pub async fn create_satellite(
    req: CreateSatelliteRequest,
) -> Result<SatelliteResponse, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/satellites", get_satellite_service_url());
    let resp = client.post(&url).json(&req).send().await?;
    resp.json::<SatelliteResponse>().await
}

pub async fn update_satellite_status(
    id: Uuid,
    req: UpdateSatelliteStatusRequest,
) -> Result<SatelliteResponse, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/satellites/{}/status", get_satellite_service_url(), id);
    let resp = client.put(&url).json(&req).send().await?;
    resp.json::<SatelliteResponse>().await
}

pub async fn calculate_position(
    req: CalculatePositionRequest,
) -> Result<CalculatePositionResponse, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/position", get_satellite_service_url());
    let resp = client.post(&url).json(&req).send().await?;
    resp.json::<CalculatePositionResponse>().await
}
