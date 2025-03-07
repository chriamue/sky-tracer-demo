use serde::{Deserialize, Serialize};
pub mod error;

/// Represents an airport with its location and metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Airport {
    /// Unique identifier
    pub id: u32,
    /// Full name of the airport
    pub name: String,
    /// City where the airport is located
    pub city: String,
    /// Country where the airport is located
    pub country: String,
    /// IATA code (3-letter code like LAX, JFK)
    pub iata: String,
    /// ICAO code (4-letter code like KLAX, KJFK)
    pub icao: String,
    /// Latitude in decimal degrees
    pub latitude: f64,
    /// Longitude in decimal degrees
    pub longitude: f64,
    /// Altitude in feet
    pub altitude: i32,
    /// Timezone offset from UTC in hours
    pub timezone_offset: i8,
    /// Daylight savings time code
    pub dst: String,
    /// Timezone name
    pub timezone: String,
    /// Type of the location (usually "airport")
    pub r#type: String,
    /// Source of the data
    pub source: String,
}

impl Airport {
    /// Returns the position as (latitude, longitude, altitude in feet)
    pub fn position(&self) -> (f64, f64, i32) {
        (self.latitude, self.longitude, self.altitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_airport_creation() {
        let airport = Airport {
            id: 1,
            name: "Test Airport".to_string(),
            city: "Test City".to_string(),
            country: "Test Country".to_string(),
            iata: "TST".to_string(),
            icao: "TTST".to_string(),
            latitude: 12.345,
            longitude: -67.890,
            altitude: 1234,
            timezone_offset: 2,
            dst: "E".to_string(),
            timezone: "Europe/TestCity".to_string(),
            r#type: "airport".to_string(),
            source: "Test Source".to_string(),
        };

        assert_eq!(airport.id, 1);
        assert_eq!(airport.name, "Test Airport");
        assert_eq!(airport.city, "Test City");
        assert_eq!(airport.country, "Test Country");
        assert_eq!(airport.iata, "TST");
        assert_eq!(airport.icao, "TTST");
        assert_eq!(airport.latitude, 12.345);
        assert_eq!(airport.longitude, -67.890);
        assert_eq!(airport.altitude, 1234);
        assert_eq!(airport.timezone_offset, 2);
        assert_eq!(airport.dst, "E");
        assert_eq!(airport.timezone, "Europe/TestCity");
        assert_eq!(airport.r#type, "airport");
        assert_eq!(airport.source, "Test Source");
    }

    #[test]
    fn test_position_method() {
        let airport = Airport {
            id: 42,
            name: "Position Test Airport".to_string(),
            city: "Position City".to_string(),
            country: "Position Country".to_string(),
            iata: "POS".to_string(),
            icao: "PPOS".to_string(),
            latitude: 23.456,
            longitude: -78.901,
            altitude: 5678,
            timezone_offset: 3,
            dst: "A".to_string(),
            timezone: "America/PositionCity".to_string(),
            r#type: "airport".to_string(),
            source: "Position Test".to_string(),
        };

        let position = airport.position();

        assert_eq!(position.0, 23.456);
        assert_eq!(position.1, -78.901);
        assert_eq!(position.2, 5678);

        // Alternative way to test the tuple
        assert_eq!(position, (23.456, -78.901, 5678));
    }

    #[test]
    fn test_serde_serialization() {
        let airport = Airport {
            id: 123,
            name: "Serialization Airport".to_string(),
            city: "Serialize City".to_string(),
            country: "Serialize Country".to_string(),
            iata: "SER".to_string(),
            icao: "SERI".to_string(),
            latitude: 34.567,
            longitude: -89.012,
            altitude: 9012,
            timezone_offset: 4,
            dst: "S".to_string(),
            timezone: "Pacific/SerializeCity".to_string(),
            r#type: "airport".to_string(),
            source: "Serialization Test".to_string(),
        };

        // Test serialization
        let json = serde_json::to_string(&airport).expect("Failed to serialize airport");

        // Test deserialization
        let deserialized: Airport =
            serde_json::from_str(&json).expect("Failed to deserialize airport");

        assert_eq!(deserialized.id, airport.id);
        assert_eq!(deserialized.name, airport.name);
        assert_eq!(deserialized.city, airport.city);
        assert_eq!(deserialized.country, airport.country);
        assert_eq!(deserialized.iata, airport.iata);
        assert_eq!(deserialized.icao, airport.icao);
        assert_eq!(deserialized.latitude, airport.latitude);
        assert_eq!(deserialized.longitude, airport.longitude);
        assert_eq!(deserialized.altitude, airport.altitude);
        assert_eq!(deserialized.timezone_offset, airport.timezone_offset);
        assert_eq!(deserialized.dst, airport.dst);
        assert_eq!(deserialized.timezone, airport.timezone);
        assert_eq!(deserialized.r#type, airport.r#type);
        assert_eq!(deserialized.source, airport.source);
    }

    #[test]
    fn test_clone() {
        let original = Airport {
            id: 456,
            name: "Clone Airport".to_string(),
            city: "Clone City".to_string(),
            country: "Clone Country".to_string(),
            iata: "CLN".to_string(),
            icao: "CLON".to_string(),
            latitude: 45.678,
            longitude: -90.123,
            altitude: 3456,
            timezone_offset: 5,
            dst: "C".to_string(),
            timezone: "Europe/CloneCity".to_string(),
            r#type: "airport".to_string(),
            source: "Clone Test".to_string(),
        };

        let cloned = original.clone();

        // Ensure the cloned object has the same values
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.name, original.name);
        assert_eq!(cloned.city, original.city);
        assert_eq!(cloned.latitude, original.latitude);
        assert_eq!(cloned.longitude, original.longitude);
        assert_eq!(cloned.altitude, original.altitude);

        // Ensure that modifying the clone doesn't affect the original
        let mut mutable_clone = original.clone();
        mutable_clone.name = "Modified Name".to_string();
        mutable_clone.latitude = 99.999;

        assert_ne!(mutable_clone.name, original.name);
        assert_ne!(mutable_clone.latitude, original.latitude);
        assert_eq!(original.name, "Clone Airport");
        assert_eq!(original.latitude, 45.678);
    }
}
