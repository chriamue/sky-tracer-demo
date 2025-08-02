use reqwest_middleware::ClientWithMiddleware;
use sky_tracer::protocol::{
    airports::SearchAirportsResponse,
    flights::{FlightPositionResponse, FlightResponse},
    AIRPORTS_SEARCH_API_PATH, BABEL_API_PATH,
};
use thiserror::Error;
use tracing::{error, info, instrument, warn};

#[derive(Error, Debug)]
pub enum DelayServiceError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest_middleware::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] reqwest::Error),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("Airport not found: {0}")]
    AirportNotFound(String),
}

#[derive(Clone)]
pub struct DelayService {
    client: ClientWithMiddleware,
    tower_babel_base_url: String,
    airport_service_base_url: String,
}

impl DelayService {
    pub fn new(
        client: ClientWithMiddleware,
        tower_babel_base_url: String,
        airport_service_base_url: String,
    ) -> Self {
        Self {
            client,
            tower_babel_base_url,
            airport_service_base_url,
        }
    }

    #[instrument(skip(self), fields(airport_code = %airport_code))]
    pub async fn get_airport_position(
        &self,
        airport_code: &str,
    ) -> Result<Option<(f64, f64)>, DelayServiceError> {
        info!("Fetching position for airport: {}", airport_code);

        // Use the protocol constant for the airport search API path
        let url = format!(
            "{}{}",
            self.airport_service_base_url, AIRPORTS_SEARCH_API_PATH
        );

        let response = self
            .client
            .get(&url)
            .query(&[("code", airport_code)])
            .send()
            .await?;

        if response.status().is_success() {
            let search_response = response.json::<SearchAirportsResponse>().await?;

            if let Some(airport) = search_response.airports.first() {
                info!(
                    airport_code = %airport_code,
                    airport_name = %airport.name,
                    lat = airport.position.latitude,
                    lon = airport.position.longitude,
                    "Found airport position"
                );
                Ok(Some((
                    airport.position.latitude,
                    airport.position.longitude,
                )))
            } else {
                warn!("Airport not found: {}", airport_code);
                Ok(None)
            }
        } else {
            error!(
                "Failed to fetch airport position: HTTP {}",
                response.status()
            );
            Err(DelayServiceError::AirportNotFound(airport_code.to_string()))
        }
    }

    #[instrument(skip(self), fields(airport_code = %airport_code))]
    pub async fn get_flights_by_airport(
        &self,
        airport_code: &str,
    ) -> Result<Vec<FlightResponse>, DelayServiceError> {
        info!("Fetching flights for airport: {}", airport_code);

        // Use the protocol constant for the babel API path
        let url = format!(
            "{}{}/{}",
            self.tower_babel_base_url, BABEL_API_PATH, airport_code
        );

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let flights = response.json::<Vec<FlightResponse>>().await?;
            info!(
                "Retrieved {} flights for airport {}",
                flights.len(),
                airport_code
            );
            Ok(flights)
        } else {
            error!("Failed to fetch flights: HTTP {}", response.status());
            Ok(Vec::new())
        }
    }

    #[instrument(skip(self), fields(flight_number = %flight_number))]
    pub async fn get_flight_position(
        &self,
        flight_number: &str,
    ) -> Result<Option<FlightPositionResponse>, DelayServiceError> {
        info!("Fetching position for flight: {}", flight_number);

        // Use the protocol constant for the position API path
        let url = format!(
            "{}{}/{}/position",
            self.tower_babel_base_url, BABEL_API_PATH, flight_number
        );

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let position = response.json::<FlightPositionResponse>().await?;
            info!("Retrieved position for flight {}", flight_number);
            Ok(Some(position))
        } else {
            info!("No position data available for flight {}", flight_number);
            Ok(None)
        }
    }

    #[instrument(skip(self))]
    pub async fn get_flights_with_positions(
        &self,
        airport_code: &str,
    ) -> Result<Vec<(FlightResponse, Option<FlightPositionResponse>)>, DelayServiceError> {
        let flights = self.get_flights_by_airport(airport_code).await?;

        let mut flights_with_positions = Vec::new();

        for flight in flights {
            let position = self.get_flight_position(&flight.flight_number).await?;
            flights_with_positions.push((flight, position));
        }

        Ok(flights_with_positions)
    }

    /// Get complete delay information with detailed error handling
    #[instrument(skip(self), fields(airport_code = %airport_code))]
    pub async fn get_airport_delays_with_errors(
        &self,
        airport_code: &str,
    ) -> (
        Vec<(FlightResponse, Option<FlightPositionResponse>)>,
        Option<(f64, f64)>,
        Option<String>,
    ) {
        info!(
            "Getting complete delay information for airport: {}",
            airport_code
        );

        // Fetch flights and airport position concurrently
        let (flights_result, position_result) = tokio::join!(
            self.get_flights_with_positions(airport_code),
            self.get_airport_position(airport_code)
        );

        let mut error_message = None;
        let flights = match flights_result {
            Ok(flights) => flights,
            Err(e) => {
                error!("Failed to fetch flights: {:?}", e);
                error_message = Some(format!("Failed to fetch flight information: {:?}", e));
                Vec::new()
            }
        };

        let position = match position_result {
            Ok(pos) => pos,
            Err(DelayServiceError::AirportNotFound(_)) => {
                warn!("Airport {} not found", airport_code);
                if error_message.is_none() {
                    error_message = Some(format!(
                        "Airport '{}' not found. Please check the airport code.",
                        airport_code
                    ));
                }
                None
            }
            Err(e) => {
                warn!("Failed to fetch airport position: {}", e);
                // Don't override flight errors, position is optional
                None
            }
        };

        info!(
            airport_code = %airport_code,
            flights_count = flights.len(),
            has_position = position.is_some(),
            has_error = error_message.is_some(),
            "Retrieved delay information with error handling"
        );

        (flights, position, error_message)
    }
}
