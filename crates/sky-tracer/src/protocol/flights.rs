use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateFlightRequest {
    /// Aircraft registration number
    pub aircraft_number: String,
    /// Departure airport code
    pub departure: String,
    /// Arrival airport code
    pub arrival: String,
    /// Scheduled departure time (ISO 8601)
    #[schema(value_type = String, format = "date-time", example = "2025-03-01T10:00:00Z")]
    pub departure_time: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct FlightResponse {
    /// Flight number
    pub flight_number: String,
    /// Aircraft registration number
    pub aircraft_number: String,
    /// Departure airport code
    pub departure: String,
    /// Arrival airport code
    pub arrival: String,
    /// Scheduled departure time
    pub departure_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListFlightsRequest {
    /// Optional departure airport code
    pub departure: Option<String>,
    /// Optional arrival airport code
    pub arrival: Option<String>,
    /// Optional date (YYYY-MM-DD)
    pub date: Option<String>,
}
