use crate::services::airports::{fetch_airport_by_code, fetch_airports};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content},
    service::{ElicitationError, RequestContext},
    schemars, tool, tool_handler, tool_router,
};
use serde_json::json;
use tracing::{error, info};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetAirportRequest {
    #[schemars(description = "Airport code (IATA/ICAO)")]
    pub code: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ResolveAirportRequest {
    #[schemars(description = "Airport code/name fragment (e.g. FRA, Frank)")]
    pub query: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SelectedAirportCode {
    #[schemars(description = "Choose one airport code from the options shown")]
    pub airport_code: String,
}

rmcp::elicit_safe!(SelectedAirportCode);

#[derive(Clone, Debug, Default)]
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

    #[tool(
        description = "List all airports",
        annotations(read_only_hint = true, destructive_hint = false, idempotent_hint = true)
    )]
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

    #[tool(
        description = "Get information about a specific airport by code",
        annotations(read_only_hint = true, destructive_hint = false, idempotent_hint = true)
    )]
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

    #[tool(
        description = "Resolve airport query; if multiple matches exist, ask the user to choose one",
        annotations(read_only_hint = true, destructive_hint = false, idempotent_hint = false, open_world_hint = true)
    )]
    pub async fn resolve_airport(
        &self,
        Parameters(req): Parameters<ResolveAirportRequest>,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let query = req.query.trim();
        if query.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(
                "Please provide a non-empty airport query.".to_string(),
            )]));
        }

        let query_upper = query.to_uppercase();
        let airports = fetch_airports().await.map_err(|e| {
            error!("Failed to fetch airports for resolve_airport: {}", e);
            McpError::internal_error(
                "Failed to fetch airports",
                Some(json!({"error": e.to_string(), "query": query})),
            )
        })?;

        let matches: Vec<_> = airports
            .into_iter()
            .filter(|a| {
                a.code.to_uppercase().contains(&query_upper)
                    || a.name.to_uppercase().contains(&query_upper)
            })
            .take(12)
            .collect();

        if matches.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(format!(
                "No airports found matching '{}'.",
                query
            ))]));
        }

        if matches.len() == 1 {
            let airport = &matches[0];
            return Ok(CallToolResult::success(vec![Content::text(format!(
                "Airport: {}\nCode: {}\nLatitude: {}\nLongitude: {}\nID: {}\n",
                airport.name, airport.code, airport.latitude, airport.longitude, airport.id
            ))]));
        }

        let option_lines = matches
            .iter()
            .map(|a| format!("{} ({})", a.name, a.code))
            .collect::<Vec<_>>()
            .join("\n- ");

        let prompt = format!(
            "Multiple airports match '{query}'. Choose one airport_code from:\n- {option_lines}"
        );

        let chosen_code = match context.peer.elicit::<SelectedAirportCode>(prompt).await {
            Ok(Some(selection)) => selection.airport_code.trim().to_uppercase(),
            Ok(None) => {
                return Ok(CallToolResult::success(vec![Content::text(
                    "No airport was selected.".to_string(),
                )]));
            }
            Err(ElicitationError::CapabilityNotSupported) => {
                let fallback = format!(
                    "Multiple matches for '{}'. Client does not support elicitation.\n\
                     Call get_airport with one of:\n- {}",
                    query,
                    matches
                        .iter()
                        .map(|a| a.code.as_str())
                        .collect::<Vec<_>>()
                        .join("\n- ")
                );
                return Ok(CallToolResult::success(vec![Content::text(fallback)]));
            }
            Err(ElicitationError::UserDeclined | ElicitationError::UserCancelled) => {
                return Ok(CallToolResult::success(vec![Content::text(
                    "Airport selection cancelled.".to_string(),
                )]));
            }
            Err(e) => {
                error!("Elicitation failed in resolve_airport: {}", e);
                return Err(McpError::internal_error(
                    "Airport selection failed",
                    Some(json!({"error": e.to_string()})),
                ));
            }
        };

        let selected_airport = matches
            .iter()
            .find(|a| a.code.eq_ignore_ascii_case(&chosen_code))
            .ok_or_else(|| {
                McpError::invalid_params(
                    format!(
                        "Invalid airport_code '{}'. Choose one of: {}",
                        chosen_code,
                        matches
                            .iter()
                            .map(|a| a.code.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    None,
                )
            })?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Airport: {}\nCode: {}\nLatitude: {}\nLongitude: {}\nID: {}\n",
            selected_airport.name,
            selected_airport.code,
            selected_airport.latitude,
            selected_airport.longitude,
            selected_airport.id
        ))]))
    }
}

#[tool_handler]
impl ServerHandler for AirportTools {}
