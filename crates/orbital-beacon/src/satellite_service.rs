use chrono::{DateTime, Utc};
use reqwest;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_tracing::TracingMiddleware;
use sky_tracer::model::{Position, Satellite, SatelliteStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, instrument};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum SatelliteServiceError {
    #[error("No active satellites available")]
    NoActiveSatellites,
    #[error("Failed to fetch airport data: {0}")]
    AirportFetchError(#[from] reqwest_middleware::Error),
    #[error("Airport not found: {0}")]
    AirportNotFound(String),
}

#[derive(Clone)]
pub struct SatelliteService {
    satellites: Arc<RwLock<HashMap<Uuid, Satellite>>>,
    airport_service_url: String,
}

impl SatelliteService {
    pub fn new(airport_service_url: String) -> Self {
        Self {
            satellites: Arc::new(RwLock::new(HashMap::new())),
            airport_service_url,
        }
    }

    #[instrument(skip(self))]
    pub async fn create_satellite(&self, name: String) -> Satellite {
        info!(name = %name, "Creating new satellite");
        let satellite = Satellite::new(name);
        let mut satellites = self.satellites.write().await;
        satellites.insert(satellite.id, satellite.clone());
        info!(id = %satellite.id, "Satellite created");
        satellite
    }

    #[instrument(skip(self))]
    pub async fn update_status(&self, id: Uuid, status: SatelliteStatus) -> Option<Satellite> {
        info!(id = %id, status = ?status, "Updating satellite status");
        let mut satellites = self.satellites.write().await;
        if let Some(satellite) = satellites.get_mut(&id) {
            satellite.status = status;
            info!(id = %id, "Satellite status updated");
            Some(satellite.clone())
        } else {
            info!(id = %id, "Satellite not found");
            None
        }
    }

    pub async fn get_satellite(&self, id: Uuid) -> Option<Satellite> {
        let satellites = self.satellites.read().await;
        satellites.get(&id).cloned()
    }

    pub async fn list_satellites(&self) -> Vec<Satellite> {
        let satellites = self.satellites.read().await;
        satellites.values().cloned().collect()
    }

    #[instrument(skip(self))]
    pub async fn calculate_position(
        &self,
        departure_code: &str,
        arrival_code: &str,
        departure_time: DateTime<Utc>,
        arrival_time: DateTime<Utc>,
        current_time: Option<DateTime<Utc>>,
    ) -> Result<
        (
            Vec<Position>,
            Option<sky_tracer::model::Airport>,
            Option<sky_tracer::model::Airport>,
        ),
        SatelliteServiceError,
    > {
        info!(
            departure = %departure_code,
            arrival = %arrival_code,
            "Calculating flight position"
        );
        let satellites = self.satellites.read().await;
        let active_satellites: Vec<_> = satellites
            .values()
            .filter(|s| s.is_active())
            .cloned()
            .collect();

        if active_satellites.is_empty() {
            return Err(SatelliteServiceError::NoActiveSatellites);
        }

        let departure_airport_result = self.fetch_airport(departure_code).await?;
        let arrival_airport_result = self.fetch_airport(arrival_code).await?;

        let departure_airport = departure_airport_result
            .ok_or_else(|| SatelliteServiceError::AirportNotFound(departure_code.to_string()))?;
        let arrival_airport = arrival_airport_result
            .ok_or_else(|| SatelliteServiceError::AirportNotFound(arrival_code.to_string()))?;

        let now = current_time.unwrap_or_else(Utc::now);
        let total_duration = arrival_time - departure_time;
        let elapsed = now - departure_time;

        if elapsed > total_duration || elapsed < chrono::Duration::zero() {
            return Ok((vec![], Some(departure_airport), Some(arrival_airport)));
        }

        let progress = elapsed.num_seconds() as f64 / total_duration.num_seconds() as f64;
        let current_lat = departure_airport.latitude
            + (arrival_airport.latitude - departure_airport.latitude) * progress;
        let current_lon = departure_airport.longitude
            + (arrival_airport.longitude - departure_airport.longitude) * progress;
        let altitude = 10000.0;

        let positions: Vec<Position> = active_satellites
            .iter()
            .map(|satellite| Position::new(current_lat, current_lon, altitude as f32, satellite.id))
            .collect();

        Ok((positions, Some(departure_airport), Some(arrival_airport)))
    }

    #[instrument(skip(self))]
    async fn fetch_airport(
        &self,
        code: &str,
    ) -> Result<Option<sky_tracer::model::Airport>, reqwest_middleware::Error> {
        info!(code = %code, "Fetching airport information");
        let url = format!(
            "{}/api/airports/search?code={}",
            self.airport_service_url, code
        );
        println!("Fetching airport from: {}", url);

        let client: ClientWithMiddleware = ClientBuilder::new(reqwest::Client::new())
            .with(TracingMiddleware::default())
            .build();

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let search_response = response
                .json::<sky_tracer::protocol::airports::SearchAirportsResponse>()
                .await?;

            Ok(search_response
                .airports
                .first()
                .map(|airport| sky_tracer::model::Airport {
                    id: airport.id,
                    latitude: airport.position.latitude,
                    longitude: airport.position.longitude,
                    name: airport.name.clone(),
                    code: airport.code.clone(),
                }))
        } else {
            Ok(None)
        }
    }
}
