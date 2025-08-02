use crate::models::Flight;
use chrono::{DateTime, Utc};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_tracing::TracingMiddleware;
use sky_tracer::protocol::flights::CreateFlightRequest;
use sky_tracer::protocol::satellite::{CalculatePositionRequest, CalculatePositionResponse};
use sky_tracer::protocol::SATELLITES_POSITION_API_PATH;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

#[derive(Clone, Default)]
pub struct FlightService {
    flights: Arc<RwLock<HashMap<String, Flight>>>,
    http_client: ClientWithMiddleware,
}

impl FlightService {
    pub fn new() -> Self {
        let http_client = ClientBuilder::new(reqwest::Client::new())
            .with(TracingMiddleware::default())
            .build();

        Self {
            flights: Arc::new(RwLock::new(HashMap::new())),
            http_client,
        }
    }

    /// Create a new flight
    #[instrument(skip(self), fields(
        aircraft = %request.aircraft_number,
        departure = %request.departure,
        arrival = %request.arrival
    ))]
    pub async fn create_flight(&self, request: CreateFlightRequest) -> Result<Flight, String> {
        let flight_number = self.generate_flight_number(&request.departure).await;

        let flight = Flight::new(
            flight_number.clone(),
            request.aircraft_number,
            request.departure,
            request.arrival,
            request.departure_time,
            request.arrival_time,
        );

        let mut flights = self.flights.write().await;
        flights.insert(flight_number.clone(), flight.clone());

        info!(
            flight_number = %flight_number,
            total_flights = flights.len(),
            "Flight created successfully"
        );

        Ok(flight)
    }

    /// Get a specific flight by flight number
    #[instrument(skip(self))]
    pub async fn get_flight(&self, flight_number: &str) -> Option<Flight> {
        let flights = self.flights.read().await;
        let flight = flights.get(flight_number).cloned();

        if flight.is_some() {
            debug!("Flight found: {}", flight_number);
        } else {
            debug!("Flight not found: {}", flight_number);
        }

        flight
    }

    /// List all flights with optional filters
    #[instrument(skip(self), fields(
        departure = ?departure,
        arrival = ?arrival,
        date = ?date
    ))]
    pub async fn list_flights(
        &self,
        departure: Option<String>,
        arrival: Option<String>,
        date: Option<DateTime<Utc>>,
    ) -> Vec<Flight> {
        let flights = self.flights.read().await;

        let filtered_flights: Vec<Flight> = flights
            .values()
            .filter(|flight| flight.matches_filters(departure.as_deref(), arrival.as_deref(), date))
            .cloned()
            .collect();

        info!(
            total_flights = flights.len(),
            filtered_flights = filtered_flights.len(),
            "Listed flights with filters"
        );

        filtered_flights
    }

    /// Calculate flight position using orbital beacon service
    #[instrument(skip(self))]
    pub async fn calculate_flight_position(
        &self,
        flight: &Flight,
    ) -> Result<(f64, f64, DateTime<Utc>), String> {
        // Use the correct environment variable name matching the compose file
        let orbital_beacon_url = std::env::var("ORBITAL_BEACON_BASE_URL")
            .unwrap_or_else(|_| "http://orbital-beacon:3002".to_string());

        debug!(url = %orbital_beacon_url, "Using orbital beacon URL");

        let arrival_time = flight.get_arrival_time();

        let position_request = CalculatePositionRequest {
            departure: flight.departure.clone(),
            arrival: flight.arrival.clone(),
            departure_time: flight.departure_time,
            arrival_time,
            current_time: Some(Utc::now()),
        };

        debug!(
            departure = %position_request.departure,
            arrival = %position_request.arrival,
            departure_time = %position_request.departure_time,
            arrival_time = %position_request.arrival_time,
            current_time = ?position_request.current_time,
            "Sending position calculation request"
        );

        // Serialize JSON manually for reqwest_middleware compatibility
        let json_body = match serde_json::to_string(&position_request) {
            Ok(body) => body,
            Err(e) => {
                error!(error = %e, "Failed to serialize position request");
                return Err(format!("Failed to serialize request: {}", e));
            }
        };

        // Use the orbital beacon position API path constant
        let full_url = format!("{}{}", orbital_beacon_url, SATELLITES_POSITION_API_PATH);

        info!(url = %full_url, "Sending position request to orbital beacon");

        match self
            .http_client
            .post(&full_url)
            .header("Content-Type", "application/json")
            .body(json_body)
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();
                info!(status = %status, "Received response from orbital beacon");

                if response.status().is_success() {
                    match response.json::<CalculatePositionResponse>().await {
                        Ok(position_data) => {
                            info!(
                                positions_count = position_data.positions.len(),
                                "Received position data from orbital beacon"
                            );

                            if let Some(position) = position_data.positions.first() {
                                info!(
                                    flight_number = %flight.flight_number,
                                    latitude = position.latitude,
                                    longitude = position.longitude,
                                    altitude = position.altitude,
                                    satellite_id = %position.satellite_id,
                                    "Successfully calculated flight position"
                                );

                                Ok((position.latitude, position.longitude, position.timestamp))
                            } else {
                                warn!("No position data available from orbital beacon");
                                Err("No position data available".to_string())
                            }
                        }
                        Err(e) => {
                            error!(error = %e, "Failed to parse position response");
                            Err(format!("Failed to parse position response: {}", e))
                        }
                    }
                } else {
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    error!(
                        status = %status,
                        error = %error_text,
                        "Orbital beacon returned error"
                    );
                    Err(format!("Orbital beacon error ({}): {}", status, error_text))
                }
            }
            Err(e) => {
                error!(error = %e, url = %full_url, "Failed to connect to orbital beacon");
                Err(format!("Failed to connect to orbital beacon: {}", e))
            }
        }
    }

    /// Generate a unique flight number based on departure airport
    async fn generate_flight_number(&self, departure: &str) -> String {
        let flights = self.flights.read().await;
        let count = flights.len() as u32;
        format!("{}{:04}", departure.to_uppercase(), count + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_flight() {
        let service = FlightService::new();
        let request = CreateFlightRequest {
            aircraft_number: "D-ABCD".to_string(),
            departure: "FRA".to_string(),
            arrival: "LIS".to_string(),
            departure_time: Utc::now(),
            arrival_time: None,
        };

        let result = service.create_flight(request).await;
        assert!(result.is_ok());

        let flight = result.unwrap();
        assert_eq!(flight.aircraft_number, "D-ABCD");
        assert_eq!(flight.departure, "FRA");
        assert_eq!(flight.arrival, "LIS");
        assert_eq!(flight.flight_number, "FRA0001");
    }

    #[tokio::test]
    async fn test_get_flight() {
        let service = FlightService::new();
        let request = CreateFlightRequest {
            aircraft_number: "D-ABCD".to_string(),
            departure: "FRA".to_string(),
            arrival: "LIS".to_string(),
            departure_time: Utc::now(),
            arrival_time: None,
        };

        let created_flight = service.create_flight(request).await.unwrap();
        let retrieved_flight = service.get_flight(&created_flight.flight_number).await;

        assert!(retrieved_flight.is_some());
        assert_eq!(
            retrieved_flight.unwrap().flight_number,
            created_flight.flight_number
        );
    }

    #[tokio::test]
    async fn test_list_flights_with_filters() {
        let service = FlightService::new();

        // Create test flights
        let request1 = CreateFlightRequest {
            aircraft_number: "D-ABCD".to_string(),
            departure: "FRA".to_string(),
            arrival: "LIS".to_string(),
            departure_time: Utc::now(),
            arrival_time: None,
        };

        let request2 = CreateFlightRequest {
            aircraft_number: "D-EFGH".to_string(),
            departure: "CDG".to_string(),
            arrival: "MAD".to_string(),
            departure_time: Utc::now(),
            arrival_time: None,
        };

        service.create_flight(request1).await.unwrap();
        service.create_flight(request2).await.unwrap();

        // Test no filters
        let all_flights = service.list_flights(None, None, None).await;
        assert_eq!(all_flights.len(), 2);

        // Test departure filter
        let fra_flights = service
            .list_flights(Some("FRA".to_string()), None, None)
            .await;
        assert_eq!(fra_flights.len(), 1);
        assert_eq!(fra_flights[0].departure, "FRA");

        // Test arrival filter
        let lis_flights = service
            .list_flights(None, Some("LIS".to_string()), None)
            .await;
        assert_eq!(lis_flights.len(), 1);
        assert_eq!(lis_flights[0].arrival, "LIS");
    }

    #[test]
    fn test_api_path_constants() {
        // Verify we're using the correct API paths
        use sky_tracer::protocol::SATELLITES_POSITION_API_PATH;
        assert_eq!(SATELLITES_POSITION_API_PATH, "/api/v1/satellites/position");
    }
}
