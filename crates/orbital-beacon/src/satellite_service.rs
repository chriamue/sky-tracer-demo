use chrono::{DateTime, Utc};
use reqwest;
use sky_tracer::model::{Position, Satellite, SatelliteStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

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

    pub async fn create_satellite(&self, name: String) -> Satellite {
        let satellite = Satellite::new(name);
        let mut satellites = self.satellites.write().await;
        satellites.insert(satellite.id, satellite.clone());
        satellite
    }

    pub async fn update_status(&self, id: Uuid, status: SatelliteStatus) -> Option<Satellite> {
        let mut satellites = self.satellites.write().await;
        if let Some(satellite) = satellites.get_mut(&id) {
            satellite.status = status;
            Some(satellite.clone())
        } else {
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

    pub async fn calculate_position(
        &self,
        departure_code: &str,
        arrival_code: &str,
        departure_time: DateTime<Utc>,
        arrival_time: DateTime<Utc>,
        current_time: Option<DateTime<Utc>>,
    ) -> (
        Vec<Position>,
        Option<sky_tracer::model::Airport>,
        Option<sky_tracer::model::Airport>,
    ) {
        let satellites = self.satellites.read().await;
        let active_satellites: Vec<_> = satellites
            .values()
            .filter(|s| s.is_active())
            .cloned()
            .collect();

        if active_satellites.is_empty() {
            return (vec![], None, None);
        }

        let departure_airport_result = self.fetch_airport(departure_code).await;
        let arrival_airport_result = self.fetch_airport(arrival_code).await;

        match (departure_airport_result, arrival_airport_result) {
            (Ok(Some(dep)), Ok(Some(arr))) => {
                let now = current_time.unwrap_or_else(Utc::now);
                let total_duration = arrival_time - departure_time;
                let elapsed = now - departure_time;

                if elapsed > total_duration || elapsed < chrono::Duration::zero() {
                    return (vec![], Some(dep), Some(arr));
                }

                let progress = elapsed.num_seconds() as f64 / total_duration.num_seconds() as f64;
                let current_lat = dep.latitude + (arr.latitude - dep.latitude) * progress;
                let current_lon = dep.longitude + (arr.longitude - dep.longitude) * progress;
                let altitude = 10000.0;

                let positions: Vec<Position> = active_satellites
                    .iter()
                    .map(|satellite| {
                        Position::new(current_lat, current_lon, altitude as f32, satellite.id)
                    })
                    .collect();
                (positions, Some(dep), Some(arr))
            }
            (Err(e), _) => {
                println!("Failed to fetch departure airport data: {}", e);
                (vec![], None, None)
            }
            (_, Err(e)) => {
                println!("Failed to fetch arrival airport data: {}", e);
                (vec![], None, None)
            }
            _ => {
                println!("Departure or arrival airport not found");
                (vec![], None, None)
            }
        }
    }

    async fn fetch_airport(
        &self,
        code: &str,
    ) -> Result<Option<sky_tracer::model::Airport>, reqwest::Error> {
        let url = format!(
            "{}/api/airports/search?code={}",
            self.airport_service_url, code
        );
        println!("Fetching airport from: {}", url);

        let response = reqwest::get(&url).await?;

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
