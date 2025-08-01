use crate::services::flights::{create_flight, fetch_flight_by_number, fetch_flights};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;
use serde_json::json;
use sky_tracer::model::flight::Flight;
use sky_tracer::protocol::flights::CreateFlightRequest;
use tracing::{error, info};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetFlightRequest {
    #[schemars(description = "Flight number")]
    pub flight_number: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateFlightToolRequest {
    #[schemars(description = "Aircraft registration number")]
    pub aircraft_number: String,
    #[schemars(description = "Departure airport code")]
    pub departure: String,
    #[schemars(description = "Arrival airport code")]
    pub arrival: String,
    #[schemars(description = "Departure time (RFC3339)")]
    pub departure_time: String,
    #[schemars(description = "Arrival time (RFC3339, optional)")]
    pub arrival_time: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListFlightsToolRequest {
    #[schemars(description = "Filter by departure airport code (optional)")]
    pub departure: Option<String>,
    #[schemars(description = "Filter by arrival airport code (optional)")]
    pub arrival: Option<String>,
    #[schemars(description = "Filter by date (YYYY-MM-DD, optional)")]
    pub date: Option<String>,
}

#[derive(Clone, Debug)]
pub struct FlightTools {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl FlightTools {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "List all flights with optional filters")]
    pub async fn list_flights(
        &self,
        Parameters(req): Parameters<ListFlightsToolRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!(
            "Listing flights with filters: departure={:?}, arrival={:?}, date={:?}",
            req.departure, req.arrival, req.date
        );

        let flights = fetch_flights().await.map_err(|e| {
            error!("Failed to fetch flights: {}", e);
            McpError::internal_error(
                "Failed to fetch flights",
                Some(json!({"error": e.to_string()})),
            )
        })?;

        // Apply filters
        let filtered_flights: Vec<_> = flights
            .into_iter()
            .filter(|flight| {
                let matches_departure = req.departure.as_ref().map_or(true, |dep| {
                    flight
                        .departure
                        .to_lowercase()
                        .contains(&dep.to_lowercase())
                });
                let matches_arrival = req.arrival.as_ref().map_or(true, |arr| {
                    flight.arrival.to_lowercase().contains(&arr.to_lowercase())
                });
                let matches_date = req.date.as_ref().map_or(true, |date| {
                    flight.departure_time.format("%Y-%m-%d").to_string() == *date
                });

                matches_departure && matches_arrival && matches_date
            })
            .collect();

        let mut result = String::new();
        if filtered_flights.is_empty() {
            result = "No flights found matching the criteria.".to_string();
        } else {
            result.push_str(&format!("Found {} flights:\n\n", filtered_flights.len()));
            for flight in &filtered_flights {
                result.push_str(&format!(
                    "Flight: {}\n\
                     Aircraft: {}\n\
                     Route: {} → {}\n\
                     Departure: {}\n\
                     Arrival: {}\n\n",
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
        }

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get detailed information about a specific flight")]
    pub async fn get_flight(
        &self,
        Parameters(GetFlightRequest { flight_number }): Parameters<GetFlightRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("Getting flight details for: {}", flight_number);

        let flight = fetch_flight_by_number(&flight_number).await.map_err(|e| {
            error!("Failed to fetch flight {}: {}", flight_number, e);
            McpError::internal_error(
                "Failed to fetch flight",
                Some(json!({"error": e.to_string(), "flight_number": flight_number})),
            )
        })?;

        let result = format!(
            "Flight Details:\n\
             Flight Number: {}\n\
             Aircraft: {}\n\
             Route: {} → {}\n\
             Departure Time: {}\n\
             Arrival Time: {}\n\
             Status: Active",
            flight.flight_number,
            flight.aircraft_number,
            flight.departure,
            flight.arrival,
            flight.departure_time.format("%Y-%m-%d %H:%M UTC"),
            flight
                .arrival_time
                .map(|t| t.format("%Y-%m-%d %H:%M UTC").to_string())
                .unwrap_or_else(|| "TBD".to_string())
        );

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Create a new flight")]
    pub async fn create_flight(
        &self,
        Parameters(req): Parameters<CreateFlightToolRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("Creating new flight: {} → {}", req.departure, req.arrival);

        // Parse departure time
        let departure_time = chrono::DateTime::parse_from_rfc3339(&req.departure_time)
            .map_err(|e| {
                McpError::invalid_params(
                    "Invalid departure_time format",
                    Some(json!({"error": e.to_string()})),
                )
            })?
            .with_timezone(&chrono::Utc);

        // Parse arrival time if provided
        let arrival_time = req
            .arrival_time
            .map(|at| {
                chrono::DateTime::parse_from_rfc3339(&at)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .map_err(|e| {
                        McpError::invalid_params(
                            "Invalid arrival_time format",
                            Some(json!({"error": e.to_string()})),
                        )
                    })
            })
            .transpose()?;

        let create_request = CreateFlightRequest {
            aircraft_number: req.aircraft_number,
            departure: req.departure,
            arrival: req.arrival,
            departure_time,
            arrival_time,
        };

        // Convert to Flight model for the create_flight service
        let flight = Flight {
            flight_number: format!("{}0001", create_request.departure), // Temporary, will be overwritten
            aircraft_number: create_request.aircraft_number,
            departure: create_request.departure,
            arrival: create_request.arrival,
            departure_time: create_request.departure_time,
            arrival_time: create_request.arrival_time,
        };

        let created_flight = create_flight(flight).await.map_err(|e| {
            error!("Failed to create flight: {}", e);
            McpError::internal_error(
                "Failed to create flight",
                Some(json!({"error": e.to_string()})),
            )
        })?;

        let result = format!(
            "Flight created successfully!\n\
             Flight Number: {}\n\
             Aircraft: {}\n\
             Route: {} → {}\n\
             Departure: {}\n\
             Arrival: {}",
            created_flight.flight_number,
            created_flight.aircraft_number,
            created_flight.departure,
            created_flight.arrival,
            created_flight.departure_time.format("%Y-%m-%d %H:%M UTC"),
            created_flight
                .arrival_time
                .map(|t| t.format("%Y-%m-%d %H:%M UTC").to_string())
                .unwrap_or_else(|| "TBD".to_string())
        );

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Search flights by route")]
    pub async fn search_flights_by_route(
        &self,
        Parameters(req): Parameters<ListFlightsToolRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!(
            "Searching flights by route: {:?} → {:?}",
            req.departure, req.arrival
        );

        // This is essentially the same as list_flights but with a different description
        self.list_flights(Parameters(req)).await
    }
}

#[tool_handler]
impl ServerHandler for FlightTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "sky-nexus-mcp-flights".to_string(),
                version: "0.1.0".to_string(),
            },
            instructions: Some(
                "Flight tools for Sky Nexus:\n\
                - list_flights: List all flights with optional filters (departure, arrival, date)\n\
                - get_flight: Get detailed information about a specific flight by flight number\n\
                - create_flight: Create a new flight with aircraft, route, and schedule details\n\
                - search_flights_by_route: Search flights by departure and arrival airports"
                    .to_string(),
            ),
        }
    }
}
