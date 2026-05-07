use crate::services::flights::fetch_flights;
use askama::Template;
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content, Implementation, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FlightDashboardRequest {
    #[schemars(description = "Filter by departure airport code (optional)")]
    pub departure: Option<String>,
    #[schemars(description = "Filter by arrival airport code (optional)")]
    pub arrival: Option<String>,
}

#[derive(Serialize)]
pub struct FlightRow {
    pub flight_number: String,
    pub aircraft_number: String,
    pub departure: String,
    pub arrival: String,
    pub departure_time: String,
    pub arrival_time: String,
}

#[derive(Template)]
#[template(path = "flight_dashboard.html", escape = "none")]
pub struct FlightDashboardTemplate;

impl FlightDashboardTemplate {
    pub fn render_html() -> Result<String, askama::Error> {
        Self.render()
    }
}

#[derive(Clone, Debug, Default)]
pub struct FlightDashboardTools {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl FlightDashboardTools {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Show an interactive flight dashboard panel with a Leaflet map. Use when the user asks to see, show, display, or track flights — optionally filtered by departure or arrival airport code."
    )]
    pub async fn flight_dashboard(
        &self,
        Parameters(req): Parameters<FlightDashboardRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!(
            departure = ?req.departure,
            arrival = ?req.arrival,
            "Fetching flights for dashboard"
        );

        let all_flights = fetch_flights().await.map_err(|e| {
            error!("Failed to fetch flights: {}", e);
            McpError::internal_error(
                "Failed to fetch flights",
                Some(json!({ "error": e.to_string() })),
            )
        })?;

        let filtered: Vec<FlightRow> = all_flights
            .into_iter()
            .filter(|f| {
                req.departure
                    .as_ref()
                    .is_none_or(|d| f.departure.to_uppercase().contains(&d.to_uppercase()))
                    && req
                        .arrival
                        .as_ref()
                        .is_none_or(|a| f.arrival.to_uppercase().contains(&a.to_uppercase()))
            })
            .map(|f| FlightRow {
                flight_number: f.flight_number,
                aircraft_number: f.aircraft_number,
                departure: f.departure,
                arrival: f.arrival,
                departure_time: f.departure_time.format("%Y-%m-%d %H:%M UTC").to_string(),
                arrival_time: f
                    .arrival_time
                    .map(|t| t.format("%Y-%m-%d %H:%M UTC").to_string())
                    .unwrap_or_else(|| "TBD".to_string()),
            })
            .collect();

        // Return flight data as JSON — host pushes this into the MCP App iframe
        let payload = serde_json::to_string(&filtered).unwrap_or_else(|_| "[]".to_string());
        Ok(CallToolResult::success(vec![Content::text(payload)]))
    }
}

#[tool_handler]
impl ServerHandler for FlightDashboardTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "Flight dashboard tool — renders interactive HTML panel with Leaflet map",
            )
    }
}
