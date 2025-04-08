use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Generates an example departure time string (1 minute from now)
pub fn example_departure_time() -> String {
    let future_time = Utc::now() + Duration::minutes(1);
    future_time.to_rfc3339()
}

/// Generates an example arrival time string (10 minutes from now)
pub fn example_arrival_time() -> String {
    let future_time = Utc::now() + Duration::minutes(10);
    future_time.to_rfc3339()
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateFlightRequest {
    /// Aircraft registration number
    pub aircraft_number: String,
    /// Departure airport code
    pub departure: String,
    /// Arrival airport code
    pub arrival: String,
    /// Scheduled departure time (ISO 8601) - should be set to current time + 1 minute
    #[schema(value_type = String, format = "date-time", example = example_departure_time)]
    pub departure_time: DateTime<Utc>,
    /// Scheduled arrival time (ISO 8601) - should be set to departure time + 90 minutes
    #[schema(value_type = String, format = "date-time", example = example_arrival_time)]
    pub arrival_time: Option<DateTime<Utc>>,
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
    /// Scheduled arrival time
    pub arrival_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct FlightPositionResponse {
    /// Flight number
    pub flight_number: String,
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
    /// Time
    pub timestamp: chrono::DateTime<chrono::Utc>,
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

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}
