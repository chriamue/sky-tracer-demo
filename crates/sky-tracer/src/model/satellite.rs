use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Satellite {
    pub id: Uuid,
    pub name: String,
    pub status: SatelliteStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum SatelliteStatus {
    Active,
    Inactive,
    Maintenance,
}

impl Satellite {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            status: SatelliteStatus::Inactive,
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == SatelliteStatus::Active
    }
}
