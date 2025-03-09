use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flight {
    /// Unique flight number (e.g., "LH1234")
    pub flight_number: String,
    /// Aircraft registration number
    pub aircraft_number: String,
    /// Departure airport
    pub departure: String, // Airport code
    /// Arrival airport
    pub arrival: String, // Airport code
    /// Scheduled departure time
    pub departure_time: DateTime<Utc>,
    /// Scheduled arrival time
    pub arrival_time: Option<DateTime<Utc>>,
}
