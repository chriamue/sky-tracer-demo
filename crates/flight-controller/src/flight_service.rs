use chrono::{DateTime, Utc};
use sky_tracer::model::flight::Flight;
use sky_tracer::protocol::flights::CreateFlightRequest;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct FlightService {
    flights: Arc<RwLock<HashMap<String, Flight>>>,
}

impl FlightService {
    pub fn new() -> Self {
        Self {
            flights: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_flight(&self, request: CreateFlightRequest) -> Result<Flight, String> {
        let flight_number = self.generate_flight_number(&request.departure).await;

        let flight = Flight {
            flight_number: flight_number.clone(),
            aircraft_number: request.aircraft_number,
            departure: request.departure,
            arrival: request.arrival,
            departure_time: request.departure_time,
            arrival_time: request.arrival_time,
        };

        let mut flights = self.flights.write().await;
        flights.insert(flight_number, flight.clone());

        Ok(flight)
    }

    pub async fn get_flight(&self, flight_number: &str) -> Option<Flight> {
        let flights = self.flights.read().await;
        flights.get(flight_number).cloned()
    }

    pub async fn list_flights(
        &self,
        departure: Option<String>,
        arrival: Option<String>,
        date: Option<DateTime<Utc>>,
    ) -> Vec<Flight> {
        let flights = self.flights.read().await;

        flights
            .values()
            .filter(|flight| {
                let matches_departure = departure
                    .as_ref()
                    .map_or(true, |dep| flight.departure == *dep);
                let matches_arrival = arrival.as_ref().map_or(true, |arr| flight.arrival == *arr);
                let matches_date = date.as_ref().map_or(true, |date| {
                    flight.departure_time.date_naive() == date.date_naive()
                });

                matches_departure && matches_arrival && matches_date
            })
            .cloned()
            .collect()
    }

    async fn generate_flight_number(&self, departure: &str) -> String {
        let flights = self.flights.read().await;
        let count = flights.len() as u32;
        format!("{}{:04}", departure, count + 1)
    }
}
