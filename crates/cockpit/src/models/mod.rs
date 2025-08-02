use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Airport {
    pub code: String,
    pub position: (f64, f64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flight {
    pub flight_number: String,
    pub departure: Airport,
    pub arrival: Airport,
    pub position: Option<(f64, f64)>,
}

impl Airport {
    pub fn new(code: String, position: (f64, f64)) -> Self {
        Self { code, position }
    }
}

impl Flight {
    pub fn new(
        flight_number: String,
        departure: Airport,
        arrival: Airport,
        position: Option<(f64, f64)>,
    ) -> Self {
        Self {
            flight_number,
            departure,
            arrival,
            position,
        }
    }

    pub fn with_position(mut self, position: (f64, f64)) -> Self {
        self.position = Some(position);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_airport_creation() {
        let airport = Airport::new("FRA".to_string(), (50.033333, 8.570556));
        assert_eq!(airport.code, "FRA");
        assert_eq!(airport.position, (50.033333, 8.570556));
    }

    #[test]
    fn test_flight_creation() {
        let departure = Airport::new("FRA".to_string(), (50.033333, 8.570556));
        let arrival = Airport::new("LIS".to_string(), (38.7613, -9.1357));

        let flight = Flight::new("LH1234".to_string(), departure, arrival, None);

        assert_eq!(flight.flight_number, "LH1234");
        assert_eq!(flight.departure.code, "FRA");
        assert_eq!(flight.arrival.code, "LIS");
        assert_eq!(flight.position, None);
    }

    #[test]
    fn test_flight_with_position() {
        let departure = Airport::new("FRA".to_string(), (50.033333, 8.570556));
        let arrival = Airport::new("LIS".to_string(), (38.7613, -9.1357));

        let flight =
            Flight::new("LH1234".to_string(), departure, arrival, None).with_position((45.0, 0.0));

        assert_eq!(flight.position, Some((45.0, 0.0)));
    }
}
