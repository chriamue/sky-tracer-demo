use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Internal flight model for the flight controller service
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flight {
    /// Unique flight number (e.g., "LH1234")
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

impl Flight {
    pub fn new(
        flight_number: String,
        aircraft_number: String,
        departure: String,
        arrival: String,
        departure_time: DateTime<Utc>,
        arrival_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            flight_number,
            aircraft_number,
            departure,
            arrival,
            departure_time,
            arrival_time,
        }
    }

    /// Get the default arrival time (2 hours after departure if not specified)
    pub fn get_arrival_time(&self) -> DateTime<Utc> {
        self.arrival_time
            .unwrap_or_else(|| self.departure_time + chrono::Duration::hours(2))
    }

    /// Check if the flight matches the given filters
    pub fn matches_filters(
        &self,
        departure: Option<&str>,
        arrival: Option<&str>,
        date: Option<DateTime<Utc>>,
    ) -> bool {
        let matches_departure =
            departure.is_none_or(|dep| self.departure.eq_ignore_ascii_case(dep));
        let matches_arrival = arrival.is_none_or(|arr| self.arrival.eq_ignore_ascii_case(arr));
        let matches_date =
            date.is_none_or(|date| self.departure_time.date_naive() == date.date_naive());

        matches_departure && matches_arrival && matches_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flight_creation() {
        let departure_time = Utc::now();
        let arrival_time = Some(departure_time + chrono::Duration::hours(2));

        let flight = Flight::new(
            "LH1234".to_string(),
            "D-ABCD".to_string(),
            "FRA".to_string(),
            "LIS".to_string(),
            departure_time,
            arrival_time,
        );

        assert_eq!(flight.flight_number, "LH1234");
        assert_eq!(flight.aircraft_number, "D-ABCD");
        assert_eq!(flight.departure, "FRA");
        assert_eq!(flight.arrival, "LIS");
        assert_eq!(flight.departure_time, departure_time);
        assert_eq!(flight.arrival_time, arrival_time);
    }

    #[test]
    fn test_get_arrival_time_with_explicit_time() {
        let departure_time = Utc::now();
        let arrival_time = Some(departure_time + chrono::Duration::hours(3));

        let flight = Flight::new(
            "LH1234".to_string(),
            "D-ABCD".to_string(),
            "FRA".to_string(),
            "LIS".to_string(),
            departure_time,
            arrival_time,
        );

        assert_eq!(flight.get_arrival_time(), arrival_time.unwrap());
    }

    #[test]
    fn test_get_arrival_time_default() {
        let departure_time = Utc::now();
        let expected_arrival = departure_time + chrono::Duration::hours(2);

        let flight = Flight::new(
            "LH1234".to_string(),
            "D-ABCD".to_string(),
            "FRA".to_string(),
            "LIS".to_string(),
            departure_time,
            None,
        );

        assert_eq!(flight.get_arrival_time(), expected_arrival);
    }

    #[test]
    fn test_matches_filters() {
        let departure_time = Utc::now();

        let flight = Flight::new(
            "LH1234".to_string(),
            "D-ABCD".to_string(),
            "FRA".to_string(),
            "LIS".to_string(),
            departure_time,
            None,
        );

        // Test departure filter
        assert!(flight.matches_filters(Some("FRA"), None, None));
        assert!(flight.matches_filters(Some("fra"), None, None)); // Case insensitive
        assert!(!flight.matches_filters(Some("CDG"), None, None));

        // Test arrival filter
        assert!(flight.matches_filters(None, Some("LIS"), None));
        assert!(flight.matches_filters(None, Some("lis"), None)); // Case insensitive
        assert!(!flight.matches_filters(None, Some("MAD"), None));

        // Test date filter
        assert!(flight.matches_filters(None, None, Some(departure_time)));
        let different_date = departure_time + chrono::Duration::days(1);
        assert!(!flight.matches_filters(None, None, Some(different_date)));

        // Test no filters (should match)
        assert!(flight.matches_filters(None, None, None));

        // Test combined filters
        assert!(flight.matches_filters(Some("FRA"), Some("LIS"), Some(departure_time)));
        assert!(!flight.matches_filters(Some("CDG"), Some("LIS"), Some(departure_time)));
    }
}
