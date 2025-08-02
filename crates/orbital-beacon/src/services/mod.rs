use crate::models::{FlightPositionRequest, PositionCalculation};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_tracing::TracingMiddleware;
use sky_tracer::model::{Position, Satellite, SatelliteStatus};
use sky_tracer::protocol::AIRPORTS_SEARCH_API_PATH;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum SatelliteServiceError {
    #[error("No active satellites available")]
    NoActiveSatellites,
    #[error("Failed to fetch airport data: {0}")]
    AirportFetchError(#[from] reqwest_middleware::Error),
    #[error("Airport not found: {0}")]
    AirportNotFound(String),
    #[error("Invalid satellite ID: {0}")]
    InvalidSatelliteId(String),
}

#[derive(Clone)]
pub struct SatelliteService {
    satellites: Arc<RwLock<HashMap<Uuid, Satellite>>>,
    airport_service_url: String,
    http_client: ClientWithMiddleware,
}

impl SatelliteService {
    pub fn new(airport_service_url: String) -> Self {
        let http_client = ClientBuilder::new(reqwest::Client::new())
            .with(TracingMiddleware::default())
            .build();

        Self {
            satellites: Arc::new(RwLock::new(HashMap::new())),
            airport_service_url,
            http_client,
        }
    }

    /// Create a new satellite
    #[instrument(skip(self))]
    pub async fn create_satellite(&self, name: String) -> Result<Satellite, SatelliteServiceError> {
        info!(name = %name, "Creating new satellite");
        let satellite = Satellite::new(name);
        let mut satellites = self.satellites.write().await;
        satellites.insert(satellite.id, satellite.clone());
        info!(id = %satellite.id, name = %satellite.name, "Satellite created successfully");
        Ok(satellite)
    }

    /// Update satellite status
    #[instrument(skip(self))]
    pub async fn update_satellite_status(
        &self,
        id: Uuid,
        status: SatelliteStatus,
    ) -> Result<Satellite, SatelliteServiceError> {
        info!(id = %id, status = ?status, "Updating satellite status");
        let mut satellites = self.satellites.write().await;

        match satellites.get_mut(&id) {
            Some(satellite) => {
                satellite.status = status;
                info!(id = %id, name = %satellite.name, status = ?status, "Satellite status updated successfully");
                Ok(satellite.clone())
            }
            None => {
                warn!(id = %id, "Satellite not found for status update");
                Err(SatelliteServiceError::InvalidSatelliteId(id.to_string()))
            }
        }
    }

    /// Get a specific satellite by ID
    #[instrument(skip(self))]
    pub async fn get_satellite(&self, id: Uuid) -> Option<Satellite> {
        debug!(id = %id, "Retrieving satellite by ID");
        let satellites = self.satellites.read().await;
        let satellite = satellites.get(&id).cloned();

        if satellite.is_some() {
            debug!(id = %id, "Satellite found");
        } else {
            debug!(id = %id, "Satellite not found");
        }

        satellite
    }

    /// List all satellites
    #[instrument(skip(self))]
    pub async fn list_satellites(&self) -> Vec<Satellite> {
        debug!("Listing all satellites");
        let satellites = self.satellites.read().await;
        let satellite_list: Vec<Satellite> = satellites.values().cloned().collect();

        let active_count = satellite_list.iter().filter(|s| s.is_active()).count();
        info!(
            total_satellites = satellite_list.len(),
            active_satellites = active_count,
            "Retrieved satellite list"
        );

        satellite_list
    }

    /// Calculate flight position using active satellites
    #[instrument(skip(self))]
    pub async fn calculate_flight_position(
        &self,
        request: FlightPositionRequest,
    ) -> Result<PositionCalculation, SatelliteServiceError> {
        info!(
            departure = %request.departure_code,
            arrival = %request.arrival_code,
            "Calculating flight position"
        );

        // Check for active satellites
        let satellites = self.satellites.read().await;
        let active_satellites: Vec<_> = satellites
            .values()
            .filter(|s| s.is_active())
            .cloned()
            .collect();

        if active_satellites.is_empty() {
            warn!("No active satellites available for position calculation");
            return Err(SatelliteServiceError::NoActiveSatellites);
        }

        info!(
            active_satellites = active_satellites.len(),
            "Found active satellites"
        );

        // Fetch airport data
        let departure_airport = self
            .fetch_airport(&request.departure_code)
            .await?
            .ok_or_else(|| {
                SatelliteServiceError::AirportNotFound(request.departure_code.clone())
            })?;

        let arrival_airport = self
            .fetch_airport(&request.arrival_code)
            .await?
            .ok_or_else(|| SatelliteServiceError::AirportNotFound(request.arrival_code.clone()))?;

        debug!(
            departure_airport = %departure_airport.name,
            arrival_airport = %arrival_airport.name,
            "Retrieved airport information"
        );

        // Calculate positions if flight is in progress
        let positions = if request.is_in_progress() {
            let progress = request.calculate_progress();
            let current_lat = departure_airport.latitude
                + (arrival_airport.latitude - departure_airport.latitude) * progress;
            let current_lon = departure_airport.longitude
                + (arrival_airport.longitude - departure_airport.longitude) * progress;
            let altitude = 10000.0; // Standard cruising altitude

            debug!(
                progress = progress,
                current_lat = current_lat,
                current_lon = current_lon,
                altitude = altitude,
                "Calculated current flight position"
            );

            active_satellites
                .iter()
                .map(|satellite| {
                    Position::new(current_lat, current_lon, altitude as f32, satellite.id)
                })
                .collect()
        } else {
            debug!("Flight not in progress, returning empty positions");
            vec![]
        };

        info!(
            positions_count = positions.len(),
            "Position calculation completed successfully"
        );

        Ok(PositionCalculation {
            positions,
            departure_airport: Some(departure_airport),
            arrival_airport: Some(arrival_airport),
        })
    }

    /// Fetch airport information from the airport service
    #[instrument(skip(self))]
    async fn fetch_airport(
        &self,
        code: &str,
    ) -> Result<Option<sky_tracer::model::Airport>, reqwest_middleware::Error> {
        info!(code = %code, "Fetching airport information");

        let url = format!(
            "{}{}?code={}",
            self.airport_service_url, AIRPORTS_SEARCH_API_PATH, code
        );

        debug!(url = %url, "Making request to airport service");

        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            match response
                .json::<sky_tracer::protocol::airports::SearchAirportsResponse>()
                .await
            {
                Ok(search_response) => {
                    if let Some(airport_response) = search_response.airports.first() {
                        let airport = sky_tracer::model::Airport {
                            id: airport_response.id,
                            latitude: airport_response.position.latitude,
                            longitude: airport_response.position.longitude,
                            name: airport_response.name.clone(),
                            code: airport_response.code.clone(),
                        };

                        info!(
                            code = %code,
                            name = %airport.name,
                            lat = airport.latitude,
                            lon = airport.longitude,
                            "Successfully retrieved airport information"
                        );

                        Ok(Some(airport))
                    } else {
                        warn!(code = %code, "Airport not found in search results");
                        Ok(None)
                    }
                }
                Err(e) => {
                    error!(code = %code, error = %e, "Failed to parse airport search response");
                    Ok(None)
                }
            }
        } else {
            warn!(
                code = %code,
                status = %response.status(),
                "Airport service returned non-success status"
            );
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[tokio::test]
    async fn test_create_satellite() {
        let service = SatelliteService::new("http://localhost:3000".to_string());
        let result = service.create_satellite("Test Satellite".to_string()).await;

        assert!(result.is_ok());
        let satellite = result.unwrap();
        assert_eq!(satellite.name, "Test Satellite");
        assert_eq!(satellite.status, SatelliteStatus::Inactive);
    }

    #[tokio::test]
    async fn test_update_satellite_status() {
        let service = SatelliteService::new("http://localhost:3000".to_string());
        let satellite = service
            .create_satellite("Test Satellite".to_string())
            .await
            .unwrap();

        let result = service
            .update_satellite_status(satellite.id, SatelliteStatus::Active)
            .await;

        assert!(result.is_ok());
        let updated_satellite = result.unwrap();
        assert_eq!(updated_satellite.status, SatelliteStatus::Active);
    }

    #[tokio::test]
    async fn test_update_nonexistent_satellite() {
        let service = SatelliteService::new("http://localhost:3000".to_string());
        let fake_id = Uuid::new_v4();

        let result = service
            .update_satellite_status(fake_id, SatelliteStatus::Active)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            SatelliteServiceError::InvalidSatelliteId(_) => {}
            _ => panic!("Expected InvalidSatelliteId error"),
        }
    }

    #[tokio::test]
    async fn test_list_satellites() {
        let service = SatelliteService::new("http://localhost:3000".to_string());

        // Initially empty
        let satellites = service.list_satellites().await;
        assert!(satellites.is_empty());

        // Add some satellites
        service
            .create_satellite("Satellite 1".to_string())
            .await
            .unwrap();
        service
            .create_satellite("Satellite 2".to_string())
            .await
            .unwrap();

        let satellites = service.list_satellites().await;
        assert_eq!(satellites.len(), 2);
    }

    #[test]
    fn test_flight_position_request_progress() {
        let departure_time = Utc::now() - Duration::hours(1);
        let arrival_time = departure_time + Duration::hours(2);
        let current_time = departure_time + Duration::hours(1); // Halfway

        let request = FlightPositionRequest::new(
            "FRA".to_string(),
            "LIS".to_string(),
            departure_time,
            arrival_time,
        )
        .with_current_time(current_time);

        assert!(request.is_in_progress());
        let progress = request.calculate_progress();
        assert!((progress - 0.5).abs() < 0.01);
    }
}
