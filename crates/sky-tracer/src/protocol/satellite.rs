use super::airports::AirportResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::{Position, SatelliteStatus};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSatelliteRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSatelliteStatusRequest {
    pub status: SatelliteStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct SatelliteResponse {
    pub id: Uuid,
    pub name: String,
    pub status: SatelliteStatus,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CalculatePositionRequest {
    pub departure: String,
    pub arrival: String,
    pub departure_time: DateTime<Utc>,
    pub arrival_time: DateTime<Utc>,
    pub current_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct CalculatePositionResponse {
    pub positions: Vec<Position>,
    pub departure_airport: Option<AirportResponse>,
    pub arrival_airport: Option<AirportResponse>,
}
