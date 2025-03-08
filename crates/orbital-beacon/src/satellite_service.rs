use chrono::{DateTime, Duration, Utc};
use sky_tracer::model::{Position, Satellite, SatelliteStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct SatelliteService {
    satellites: Arc<RwLock<HashMap<Uuid, Satellite>>>,
}

impl SatelliteService {
    pub fn new() -> Self {
        Self {
            satellites: Arc::new(RwLock::new(HashMap::new())),
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
        departure: (f64, f64),
        arrival: (f64, f64),
        departure_time: DateTime<Utc>,
        current_time: Option<DateTime<Utc>>,
    ) -> Vec<Position> {
        let satellites = self.satellites.read().await;
        let active_satellites: Vec<_> = satellites
            .values()
            .filter(|s| s.is_active())
            .cloned()
            .collect();

        if active_satellites.is_empty() {
            return vec![];
        }

        let now = current_time.unwrap_or_else(Utc::now);
        let flight_duration = Duration::hours(
            ((arrival.0 - departure.0).powi(2) + (arrival.1 - departure.1).powi(2)).sqrt() as i64,
        );
        let elapsed = now - departure_time;

        if elapsed > flight_duration || elapsed < Duration::zero() {
            return vec![];
        }

        let progress = elapsed.num_seconds() as f64 / flight_duration.num_seconds() as f64;
        let current_lat = departure.0 + (arrival.0 - departure.0) * progress;
        let current_lon = departure.1 + (arrival.1 - departure.1) * progress;
        let altitude = 10000.0;

        active_satellites
            .iter()
            .map(|satellite| Position::new(current_lat, current_lon, altitude, satellite.id))
            .collect()
    }
}
