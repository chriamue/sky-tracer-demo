use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub satellite_id: Uuid,
}

impl Position {
    pub fn new(latitude: f64, longitude: f64, altitude: f32, satellite_id: Uuid) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
            timestamp: chrono::Utc::now(),
            satellite_id,
        }
    }
}
