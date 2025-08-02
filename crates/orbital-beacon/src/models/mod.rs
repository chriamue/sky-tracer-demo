use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Position calculation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionCalculation {
    pub positions: Vec<sky_tracer::model::Position>,
    pub departure_airport: Option<sky_tracer::model::Airport>,
    pub arrival_airport: Option<sky_tracer::model::Airport>,
}

/// Flight position request parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightPositionRequest {
    pub departure_code: String,
    pub arrival_code: String,
    pub departure_time: DateTime<Utc>,
    pub arrival_time: DateTime<Utc>,
    pub current_time: Option<DateTime<Utc>>,
}

impl FlightPositionRequest {
    pub fn new(
        departure_code: String,
        arrival_code: String,
        departure_time: DateTime<Utc>,
        arrival_time: DateTime<Utc>,
    ) -> Self {
        Self {
            departure_code,
            arrival_code,
            departure_time,
            arrival_time,
            current_time: None,
        }
    }

    pub fn with_current_time(mut self, current_time: DateTime<Utc>) -> Self {
        self.current_time = Some(current_time);
        self
    }

    /// Get progress as a value between 0.0 and 1.0
    pub fn calculate_progress(&self) -> f64 {
        let now = self.current_time.unwrap_or_else(Utc::now);
        let total_duration = self.arrival_time - self.departure_time;
        let elapsed = now - self.departure_time;

        if elapsed > total_duration || elapsed < chrono::Duration::zero() {
            return if elapsed < chrono::Duration::zero() {
                0.0
            } else {
                1.0
            };
        }

        elapsed.num_seconds() as f64 / total_duration.num_seconds() as f64
    }

    /// Check if flight is currently in progress
    pub fn is_in_progress(&self) -> bool {
        let progress = self.calculate_progress();
        progress > 0.0 && progress < 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_position_calculation_creation() {
        let calculation = PositionCalculation {
            positions: vec![],
            departure_airport: None,
            arrival_airport: None,
        };

        assert!(calculation.positions.is_empty());
        assert!(calculation.departure_airport.is_none());
        assert!(calculation.arrival_airport.is_none());
    }

    #[test]
    fn test_flight_position_request_creation() {
        let departure_time = Utc::now();
        let arrival_time = departure_time + Duration::hours(2);

        let request = FlightPositionRequest::new(
            "FRA".to_string(),
            "LIS".to_string(),
            departure_time,
            arrival_time,
        );

        assert_eq!(request.departure_code, "FRA");
        assert_eq!(request.arrival_code, "LIS");
        assert_eq!(request.departure_time, departure_time);
        assert_eq!(request.arrival_time, arrival_time);
        assert!(request.current_time.is_none());
    }

    #[test]
    fn test_flight_position_request_with_current_time() {
        let departure_time = Utc::now();
        let arrival_time = departure_time + Duration::hours(2);
        let current_time = departure_time + Duration::hours(1);

        let request = FlightPositionRequest::new(
            "FRA".to_string(),
            "LIS".to_string(),
            departure_time,
            arrival_time,
        )
        .with_current_time(current_time);

        assert_eq!(request.current_time, Some(current_time));
    }

    #[test]
    fn test_calculate_progress() {
        let departure_time = Utc::now() - Duration::hours(1);
        let arrival_time = departure_time + Duration::hours(2);
        let current_time = departure_time + Duration::hours(1); // Halfway

        let request = FlightPositionRequest::new(
            "FRA".to_string(),
            "LIS".to_string(),
            departure_time,
            arrival_time,
        )
        .with_current_time(current_time);

        let progress = request.calculate_progress();
        assert!((progress - 0.5).abs() < 0.01); // Should be approximately 50%
    }

    #[test]
    fn test_is_in_progress() {
        let departure_time = Utc::now() - Duration::hours(1);
        let arrival_time = departure_time + Duration::hours(2);

        // Flight in progress
        let current_time = departure_time + Duration::minutes(30);
        let request = FlightPositionRequest::new(
            "FRA".to_string(),
            "LIS".to_string(),
            departure_time,
            arrival_time,
        )
        .with_current_time(current_time);

        assert!(request.is_in_progress());

        // Flight not started
        let current_time = departure_time - Duration::minutes(30);
        let request = request.with_current_time(current_time);
        assert!(!request.is_in_progress());

        // Flight completed
        let current_time = arrival_time + Duration::minutes(30);
        let request = request.with_current_time(current_time);
        assert!(!request.is_in_progress());
    }
}
