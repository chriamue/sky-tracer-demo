use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AirportResponse {
    /// Airport unique identifier
    pub id: u32,
    /// Airport name
    pub name: String,
    /// City where the airport is located
    pub city: String,
    /// Country where the airport is located
    pub country: String,
    /// IATA code (3-letter code)
    pub iata: String,
    /// ICAO code (4-letter code)
    pub icao: String,
    /// Airport position details
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Position {
    /// Latitude in decimal degrees
    pub latitude: f64,
    /// Longitude in decimal degrees
    pub longitude: f64,
    /// Altitude in feet
    pub altitude: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SearchAirportsRequest {
    /// Optional name to search for (matches against name, city, or country)
    pub name: Option<String>,
    /// Optional IATA code for exact match
    pub iata: Option<String>,
    /// Optional ICAO code for exact match
    pub icao: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchAirportsResponse {
    pub airports: Vec<AirportResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}
