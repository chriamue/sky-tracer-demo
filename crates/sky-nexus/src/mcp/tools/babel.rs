use crate::services::babel::{BabelServiceError, fetch_flight_position, fetch_flights_by_airport};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content},
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetFlightsByAirportRequest {
    #[schemars(description = "Airport IATA/ICAO code (e.g., JFK, LAX, LHR)")]
    pub airport_code: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetFlightPositionRequest {
    #[schemars(description = "Flight number (e.g., AA123, BA456)")]
    pub flight_number: String,
}

#[derive(Clone, Debug, Default)]
pub struct BabelTools {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl BabelTools {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Get future flights departing from or arriving at a specific airport",
        annotations(read_only_hint = true, destructive_hint = false, idempotent_hint = true)
    )]
    pub async fn get_flights_by_airport(
        &self,
        Parameters(req): Parameters<GetFlightsByAirportRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("Getting flights for airport: {}", req.airport_code);

        match fetch_flights_by_airport(&req.airport_code).await {
            Ok(flights) => {
                if flights.is_empty() {
                    let result = format!(
                        "No future flights found for airport: {}\n\
                         This could mean:\n\
                         - No flights are currently scheduled\n\
                         - All scheduled flights have already departed\n\
                         - The airport code may be incorrect",
                        req.airport_code
                    );
                    Ok(CallToolResult::success(vec![Content::text(result)]))
                } else {
                    let mut result = format!(
                        "Found {} future flights for airport {}:\n\n",
                        flights.len(),
                        req.airport_code
                    );

                    for flight in &flights {
                        result.push_str(&format!(
                            "Flight: {}\n\
                             Aircraft: {}\n\
                             Route: {} → {}\n\
                             Departure: {}\n\
                             Arrival: {}\n\
                             Status: Future Flight\n\n",
                            flight.flight_number,
                            flight.aircraft_number,
                            flight.departure,
                            flight.arrival,
                            flight.departure_time.format("%Y-%m-%d %H:%M UTC"),
                            flight
                                .arrival_time
                                .map(|t| t.format("%Y-%m-%d %H:%M UTC").to_string())
                                .unwrap_or_else(|| "TBD".to_string())
                        ));
                    }

                    Ok(CallToolResult::success(vec![Content::text(result)]))
                }
            }
            Err(BabelServiceError::NotFound(msg)) => {
                error!("Airport not found: {}", msg);
                let result = format!(
                    "Airport not found: {}\n\
                     Please check the airport code and try again.\n\
                     Examples of valid codes: JFK, LAX, LHR, CDG, NRT",
                    msg
                );
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(BabelServiceError::NoFutureFlights(airport)) => {
                let result = format!(
                    "No future flights found for airport: {}\n\
                     All scheduled flights may have already departed or \n\
                     no flights are currently scheduled from this airport.",
                    airport
                );
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                error!(
                    "Failed to fetch flights for airport {}: {}",
                    req.airport_code, e
                );
                Err(McpError::internal_error(
                    "Failed to fetch flights",
                    Some(json!({
                        "error": e.to_string(),
                        "airport_code": req.airport_code
                    })),
                ))
            }
        }
    }

    #[tool(
        description = "Get current position and status of a specific flight",
        annotations(read_only_hint = true, destructive_hint = false, idempotent_hint = true)
    )]
    pub async fn get_flight_position(
        &self,
        Parameters(req): Parameters<GetFlightPositionRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("Getting position for flight: {}", req.flight_number);

        match fetch_flight_position(&req.flight_number).await {
            Ok(position) => {
                let result = format!(
                    "Flight Position for {}:\n\
                     \n\
                     Current Location:\n\
                     - Latitude: {:.6}\n\
                     - Longitude: {:.6}\n\
                     - Last Updated: {}\n\
                     \n\
                     Status: Position tracked by Tower of Babel",
                    position.flight_number,
                    position.latitude,
                    position.longitude,
                    position.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                );

                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(BabelServiceError::NotFound(msg)) => {
                error!("Flight not found: {}", msg);
                let result = format!(
                    "Flight not found: {}\n\
                     Please check the flight number and try again.\n\
                     Examples of valid flight numbers: AA123, BA456, LH789",
                    msg
                );
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                error!(
                    "Failed to fetch flight position for {}: {}",
                    req.flight_number, e
                );
                Err(McpError::internal_error(
                    "Failed to fetch flight position",
                    Some(json!({
                        "error": e.to_string(),
                        "flight_number": req.flight_number
                    })),
                ))
            }
        }
    }

    #[tool(
        description = "Search for flights by airport code pattern",
        annotations(read_only_hint = true, destructive_hint = false, idempotent_hint = true)
    )]
    pub async fn search_flights_by_airport_pattern(
        &self,
        Parameters(req): Parameters<SearchFlightsByPatternRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("Searching flights with pattern: {}", req.pattern);

        // Try to fetch flights if the pattern looks like an airport code
        if req.pattern.len() == 3 || req.pattern.len() == 4 {
            match fetch_flights_by_airport(&req.pattern.to_uppercase()).await {
                Ok(flights) => {
                    if flights.is_empty() {
                        let result = format!(
                            "No flights found for airport pattern: {}\n\
                             Try using a complete 3-letter IATA code (e.g., JFK, LAX) \n\
                             or 4-letter ICAO code (e.g., KJFK, KLAX)",
                            req.pattern
                        );
                        Ok(CallToolResult::success(vec![Content::text(result)]))
                    } else {
                        let mut result = format!(
                            "Found {} flights matching pattern '{}':\n\n",
                            flights.len(),
                            req.pattern
                        );

                        for flight in flights.iter().take(10) {
                            // Limit to first 10 results
                            result.push_str(&format!(
                                "{}: {} → {} (Departs: {})\n",
                                flight.flight_number,
                                flight.departure,
                                flight.arrival,
                                flight.departure_time.format("%Y-%m-%d %H:%M UTC")
                            ));
                        }

                        if flights.len() > 10 {
                            result.push_str(&format!(
                                "\n... and {} more flights",
                                flights.len() - 10
                            ));
                        }

                        Ok(CallToolResult::success(vec![Content::text(result)]))
                    }
                }
                Err(_) => {
                    let result = format!(
                        "No flights found for pattern: {}\n\
                         Please try a valid airport code (e.g., JFK, LAX, LHR)",
                        req.pattern
                    );
                    Ok(CallToolResult::success(vec![Content::text(result)]))
                }
            }
        } else {
            let result = format!(
                "Invalid airport code pattern: {}\n\
                 Airport codes should be 3 letters (IATA) or 4 letters (ICAO).\n\
                 Examples: JFK, LAX, LHR, KJFK, KLAX, EGLL",
                req.pattern
            );
            Ok(CallToolResult::success(vec![Content::text(result)]))
        }
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchFlightsByPatternRequest {
    #[schemars(description = "Airport code pattern to search for")]
    pub pattern: String,
}

#[tool_handler]
impl ServerHandler for BabelTools {}
