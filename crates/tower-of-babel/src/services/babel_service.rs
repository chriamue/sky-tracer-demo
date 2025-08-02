use chrono::Utc;
use reqwest_middleware::ClientWithMiddleware;
use sky_tracer::protocol::{
    flights::{FlightPositionResponse, FlightResponse},
    FLIGHTS_API_PATH, FLIGHTS_POSITION_API_PATH,
};
use thiserror::Error;
use tracing::{debug, error, info, instrument, warn};

#[derive(Error, Debug)]
pub enum BabelServiceError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest_middleware::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] reqwest::Error),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("No future flights found for airport: {0}")]
    NoFutureFlights(String),
}

#[derive(Clone)]
pub struct BabelService {
    client: ClientWithMiddleware,
    flight_controller_base_url: String,
}

impl BabelService {
    pub fn new(client: ClientWithMiddleware, flight_controller_base_url: String) -> Self {
        Self {
            client,
            flight_controller_base_url,
        }
    }

    /// List flights by airport with future arrival filtering
    #[instrument(skip(self), fields(airport_code = %airport_code))]
    pub async fn list_flights_by_airport(
        &self,
        airport_code: &str,
    ) -> Result<Vec<FlightResponse>, BabelServiceError> {
        let url = format!(
            "{}{}?departure={}",
            self.flight_controller_base_url, FLIGHTS_API_PATH, airport_code
        );

        debug!(url = %url, airport = %airport_code, "Fetching flights");

        let response = self.client.get(&url).send().await?;
        let status = response.status();
        debug!(status = %status, "Received response");

        if response.status().is_success() {
            let all_flights: Vec<FlightResponse> = response.json().await?;

            // Filter flights with future arrival times
            let now = Utc::now();
            let future_flights: Vec<FlightResponse> = all_flights
                .into_iter()
                .filter(|flight| {
                    flight
                        .arrival_time
                        .map(|arrival| arrival > now)
                        .unwrap_or(true) // Include flights with no arrival time
                })
                .collect();

            info!(
                total_flights = future_flights.len(),
                airport = %airport_code,
                "Successfully retrieved future flights"
            );

            if future_flights.is_empty() {
                warn!(airport = %airport_code, "No future flights found");
                Err(BabelServiceError::NoFutureFlights(airport_code.to_string()))
            } else {
                Ok(future_flights)
            }
        } else if response.status().as_u16() == 404 {
            warn!(airport = %airport_code, "No flights found");
            Err(BabelServiceError::NotFound(format!(
                "No flights found for airport {}",
                airport_code
            )))
        } else {
            let error_message = response.text().await.unwrap_or_default();
            error!(
                status = %status,
                error = %error_message,
                airport = %airport_code,
                "Error from flight-controller"
            );
            Err(BabelServiceError::ServiceUnavailable(format!(
                "Flight controller returned an error: {}",
                error_message
            )))
        }
    }

    /// Get flight position
    #[instrument(skip(self), fields(flight_number = %flight_number))]
    pub async fn get_flight_position(
        &self,
        flight_number: &str,
    ) -> Result<FlightPositionResponse, BabelServiceError> {
        let url = format!(
            "{}{}",
            self.flight_controller_base_url,
            FLIGHTS_POSITION_API_PATH.replace("{flight_number}", flight_number)
        );

        debug!(
            url = %url,
            flight_number = %flight_number,
            "Fetching flight position"
        );

        let response = self.client.get(&url).send().await?;
        let status = response.status();
        debug!(status = %status, "Received response");

        if response.status().is_success() {
            let position = response.json::<FlightPositionResponse>().await?;

            info!(
                flight_number = %flight_number,
                latitude = position.latitude,
                longitude = position.longitude,
                "Successfully retrieved flight position"
            );

            Ok(position)
        } else if response.status().as_u16() == 404 {
            warn!(flight_number = %flight_number, "Flight not found");
            Err(BabelServiceError::NotFound(format!(
                "Flight not found: {}",
                flight_number
            )))
        } else {
            let error_message = response.text().await.unwrap_or_default();
            error!(
                status = %status,
                error = %error_message,
                flight_number = %flight_number,
                "Error from flight-controller"
            );
            Err(BabelServiceError::ServiceUnavailable(format!(
                "Flight controller returned an error: {}",
                error_message
            )))
        }
    }
}
