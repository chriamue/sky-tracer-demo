use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct AirportResponse {
    /// Airport unique identifier
    pub id: u32,
    /// Airport name
    pub name: String,
    /// Combined IATA/ICAO code (e.g., "LAX/KLAX")
    pub code: String,
    /// Airport position details
    pub position: Position,
}

impl From<&crate::model::airport::Airport> for AirportResponse {
    fn from(airport: &crate::model::airport::Airport) -> Self {
        let (latitude, longitude) = airport.position();
        Self {
            id: airport.id,
            name: airport.name.clone(),
            code: airport.code.clone(),
            position: Position {
                latitude,
                longitude,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Position {
    /// Latitude in decimal degrees
    pub latitude: f64,
    /// Longitude in decimal degrees
    pub longitude: f64,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SearchAirportsRequest {
    /// Optional name to search for
    pub name: Option<String>,
    /// Optional code (IATA/ICAO) for exact match
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SearchAirportsResponse {
    pub airports: Vec<AirportResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
    /// Error code for categorization
    pub code: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::airport::Airport;

    #[test]
    fn test_airport_response_conversion() {
        let airport = Airport::frankfurt();
        let response = AirportResponse::from(&airport);

        assert_eq!(response.id, airport.id);
        assert_eq!(response.name, airport.name);
        assert_eq!(response.code, airport.code);
        assert_eq!(response.position.latitude, airport.latitude);
        assert_eq!(response.position.longitude, airport.longitude);
    }
}
