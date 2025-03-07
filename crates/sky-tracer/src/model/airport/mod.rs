use serde::{Deserialize, Serialize};
pub mod error;

pub use error::AirportError;

/// Represents an airport with its essential location and identification data
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Airport {
    /// Unique identifier
    pub id: u32,
    /// Latitude in decimal degrees
    pub latitude: f64,
    /// Longitude in decimal degrees
    pub longitude: f64,
    /// Full name of the airport
    pub name: String,
    /// Combined IATA/ICAO code (e.g., "LAX/KLAX")
    pub code: String,
}

impl Default for Airport {
    fn default() -> Self {
        Self {
            id: 0,
            latitude: 0.0,
            longitude: 0.0,
            name: String::new(),
            code: String::new(),
        }
    }
}

impl Airport {
    /// Creates a new Airport instance
    pub fn new(id: u32, latitude: f64, longitude: f64, name: String, code: String) -> Self {
        Self {
            id,
            latitude,
            longitude,
            name,
            code,
        }
    }

    /// Returns the position as (latitude, longitude)
    pub fn position(&self) -> (f64, f64) {
        (self.latitude, self.longitude)
    }

    /// Creates Frankfurt Airport instance
    pub fn frankfurt() -> Self {
        Self::new(
            340,
            50.033333,
            8.570556,
            "Frankfurt am Main Airport".to_string(),
            "FRA/EDDF".to_string(),
        )
    }

    /// Creates Paris Charles de Gaulle Airport instance
    pub fn paris() -> Self {
        Self::new(
            1382,
            49.012798,
            2.55,
            "Charles de Gaulle International Airport".to_string(),
            "CDG/LFPG".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Airport {
        pub fn san_francisco() -> Self {
            Self::new(
                3469,
                37.61899948120117,
                -122.375,
                "San Francisco International Airport".to_string(),
                "SFO/KSFO".to_string(),
            )
        }

        pub fn new_york() -> Self {
            Self::new(
                3797,
                40.63980103,
                -73.77890015,
                "John F Kennedy International Airport".to_string(),
                "JFK/KJFK".to_string(),
            )
        }
    }

    #[test]
    fn test_airport_creation() {
        let airport = Airport::new(
            1,
            12.345,
            -67.890,
            "Test Airport".to_string(),
            "TST/TTST".to_string(),
        );

        assert_eq!(airport.id, 1);
        assert_eq!(airport.name, "Test Airport");
        assert_eq!(airport.latitude, 12.345);
        assert_eq!(airport.longitude, -67.890);
        assert_eq!(airport.code, "TST/TTST");
    }

    #[test]
    fn test_position_method() {
        let airport = Airport::frankfurt();
        let position = airport.position();

        assert_eq!(position, (50.033333, 8.570556));
    }

    #[test]
    fn test_predefined_airports() {
        let fra = Airport::frankfurt();
        assert_eq!(fra.code, "FRA/EDDF");

        let cdg = Airport::paris();
        assert_eq!(cdg.code, "CDG/LFPG");

        let sfo = Airport::san_francisco();
        assert_eq!(sfo.code, "SFO/KSFO");

        let jfk = Airport::new_york();
        assert_eq!(jfk.code, "JFK/KJFK");
    }

    #[test]
    fn test_serde() {
        let airport = Airport::frankfurt();
        let json = serde_json::to_string(&airport).unwrap();
        let deserialized: Airport = serde_json::from_str(&json).unwrap();
        assert_eq!(airport, deserialized);
    }
}
