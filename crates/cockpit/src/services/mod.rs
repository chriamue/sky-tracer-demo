use crate::models::{Airport, Flight};
use gloo_net::http::Request;
use sky_tracer::protocol::flights::{CreateFlightRequest, FlightPositionResponse, FlightResponse};
use sky_tracer::protocol::{AIRPORTS_SEARCH_API_PATH, FLIGHTS_API_PATH};
use std::collections::HashMap;
use tracing::{error, info, warn};

pub struct FlightService;

impl FlightService {
    /// Create a new flight
    pub async fn create_flight(request: CreateFlightRequest) -> Result<(), String> {
        match Request::post(FLIGHTS_API_PATH)
            .json(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
        {
            Ok(response) => {
                if response.ok() {
                    info!("Flight created successfully");
                    Ok(())
                } else {
                    let error_msg = format!("Server error: {}", response.status());
                    error!("{}", error_msg);
                    Err(error_msg)
                }
            }
            Err(e) => {
                let error_msg = format!("Network error: {}", e);
                error!("{}", error_msg);
                Err(error_msg)
            }
        }
    }

    /// Get all flights
    pub async fn get_flights() -> Result<Vec<FlightResponse>, String> {
        match Request::get(FLIGHTS_API_PATH).send().await {
            Ok(response) => {
                if response.ok() {
                    match response.json::<Vec<FlightResponse>>().await {
                        Ok(flights) => {
                            info!("Retrieved {} flights", flights.len());
                            Ok(flights)
                        }
                        Err(e) => {
                            let error_msg = format!("Failed to parse flights: {}", e);
                            error!("{}", error_msg);
                            Err(error_msg)
                        }
                    }
                } else {
                    let error_msg = format!("Server error: {}", response.status());
                    error!("{}", error_msg);
                    Err(error_msg)
                }
            }
            Err(e) => {
                let error_msg = format!("Network error: {}", e);
                error!("{}", error_msg);
                Err(error_msg)
            }
        }
    }

    /// Get flight position
    pub async fn get_flight_position(flight_number: &str) -> Result<Option<(f64, f64)>, String> {
        let position_url = format!("{}/{}/position", FLIGHTS_API_PATH, flight_number);

        match Request::get(&position_url).send().await {
            Ok(response) => {
                if response.ok() {
                    match response.json::<FlightPositionResponse>().await {
                        Ok(position) => Ok(Some((position.latitude, position.longitude))),
                        Err(e) => {
                            warn!("Failed to parse position for {}: {}", flight_number, e);
                            Ok(None)
                        }
                    }
                } else {
                    warn!("No position available for flight {}", flight_number);
                    Ok(None)
                }
            }
            Err(e) => {
                warn!(
                    "Network error getting position for {}: {}",
                    flight_number, e
                );
                Ok(None)
            }
        }
    }
}

pub struct AirportService;

impl AirportService {
    /// Search for airport by code
    pub async fn search_by_code(code: &str) -> Result<Option<Airport>, String> {
        let search_url = format!("{}?code={}", AIRPORTS_SEARCH_API_PATH, code);

        match Request::get(&search_url).send().await {
            Ok(response) => {
                if response.ok() {
                    match response
                        .json::<sky_tracer::protocol::airports::SearchAirportsResponse>()
                        .await
                    {
                        Ok(search_response) => {
                            if let Some(airport) = search_response.airports.first() {
                                Ok(Some(Airport::new(
                                    airport.code.clone(),
                                    (airport.position.latitude, airport.position.longitude),
                                )))
                            } else {
                                Ok(None)
                            }
                        }
                        Err(e) => {
                            let error_msg = format!("Failed to parse airport search: {}", e);
                            error!("{}", error_msg);
                            Err(error_msg)
                        }
                    }
                } else {
                    warn!("Airport not found: {}", code);
                    Ok(None)
                }
            }
            Err(e) => {
                let error_msg = format!("Network error searching for airport {}: {}", code, e);
                error!("{}", error_msg);
                Err(error_msg)
            }
        }
    }
}

pub struct DataService;

impl DataService {
    /// Get flights with their airport data and positions
    pub async fn get_flights_with_data() -> Result<(Vec<Flight>, Vec<Airport>), String> {
        let flight_responses = FlightService::get_flights().await?;
        let mut airport_map = HashMap::new();

        // Collect unique airport codes
        let airport_codes: std::collections::HashSet<String> = flight_responses
            .iter()
            .flat_map(|f| vec![f.departure.clone(), f.arrival.clone()])
            .collect();

        // Fetch all airport data
        for code in airport_codes {
            if let Ok(Some(airport)) = AirportService::search_by_code(&code).await {
                airport_map.insert(code, airport);
            }
        }

        // Convert flight responses to Flight objects
        let mut flights: Vec<Flight> = Vec::new();
        for flight_response in flight_responses {
            if let (Some(departure), Some(arrival)) = (
                airport_map.get(&flight_response.departure),
                airport_map.get(&flight_response.arrival),
            ) {
                let mut flight = Flight::new(
                    flight_response.flight_number.clone(),
                    departure.clone(),
                    arrival.clone(),
                    None,
                );

                // Try to get position
                if let Ok(Some(position)) =
                    FlightService::get_flight_position(&flight_response.flight_number).await
                {
                    flight.position = Some(position);
                }

                flights.push(flight);
            }
        }

        let airports: Vec<Airport> = airport_map.into_values().collect();

        info!(
            "Retrieved {} flights and {} airports",
            flights.len(),
            airports.len()
        );
        Ok((flights, airports))
    }

    /// Check API connection status
    pub async fn check_connection() -> bool {
        match Request::get(FLIGHTS_API_PATH).send().await {
            Ok(response) => response.ok(),
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_create_flight_request() {
        let request = CreateFlightRequest {
            aircraft_number: "LH-A320".to_string(),
            departure: "FRA".to_string(),
            arrival: "LIS".to_string(),
            departure_time: Utc::now(),
            arrival_time: Some(Utc::now() + chrono::Duration::hours(2)),
        };

        assert_eq!(request.aircraft_number, "LH-A320");
        assert_eq!(request.departure, "FRA");
        assert_eq!(request.arrival, "LIS");
    }

    #[test]
    fn test_api_paths() {
        // Ensure we're using the correct API paths
        assert_eq!(FLIGHTS_API_PATH, "/api/v1/flights");
        assert_eq!(AIRPORTS_SEARCH_API_PATH, "/api/v1/airports/search");
    }
}
