use crate::services::airports::{fetch_airport_by_code, fetch_airports};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use serde_json::json;
use tracing::{error, info};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetAirportRequest {
    #[schemars(description = "Airport code (IATA/ICAO)")]
    pub code: String,
}

#[derive(Clone, Debug)]
pub struct AirportTools {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl AirportTools {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "List all airports")]
    pub async fn list_airports(&self) -> Result<CallToolResult, McpError> {
        info!("Listing airports");

        let airports = fetch_airports().await.map_err(|e| {
            error!("Failed to fetch airports: {}", e);
            McpError::internal_error(
                "Failed to fetch airports",
                Some(json!({"error": e.to_string()})),
            )
        })?;

        let mut result = String::new();
        for airport in &airports {
            result.push_str(&format!(
                "{} ({}) - Lat: {}, Lon: {}\n",
                airport.name, airport.code, airport.latitude, airport.longitude
            ));
        }
        if result.is_empty() {
            result = "No airports found.".to_string();
        }

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get information about a specific airport by code")]
    pub async fn get_airport(
        &self,
        Parameters(GetAirportRequest { code }): Parameters<GetAirportRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("Getting airport by code: {}", code);

        let airport = fetch_airport_by_code(&code).await.map_err(|e| {
            error!("Failed to fetch airport {}: {}", code, e);
            McpError::internal_error(
                "Failed to fetch airport",
                Some(json!({"error": e.to_string(), "code": code})),
            )
        })?;

        let result = format!(
            "Airport: {}\nCode: {}\nLatitude: {}\nLongitude: {}\nID: {}\n",
            airport.name, airport.code, airport.latitude, airport.longitude, airport.id
        );

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }
}

#[tool_handler]
impl ServerHandler for AirportTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "sky-nexus-mcp-airports".to_string(),
                version: "0.1.0".to_string(),
            },
            instructions: Some(
                "Airport tools for Sky Nexus:\n\
                - list_airports: List all airports with their codes and coordinates\n\
                - get_airport: Get detailed information about a specific airport by code"
                    .to_string(),
            ),
        }
    }
}
