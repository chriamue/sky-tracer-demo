use crate::services::satellites::{
    calculate_position, create_satellite, fetch_satellites, update_satellite_status,
};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content},
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;
use serde_json::json;
use sky_tracer::protocol::satellite::{
    CalculatePositionRequest, CreateSatelliteRequest, UpdateSatelliteStatusRequest,
};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetSatelliteStatusRequest {
    #[schemars(description = "Satellite ID (UUID)")]
    pub id: Uuid,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateSatelliteToolRequest {
    #[schemars(description = "Satellite name")]
    pub name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateSatelliteStatusToolRequest {
    #[schemars(description = "Satellite ID (UUID)")]
    pub id: Uuid,
    #[schemars(description = "New status (Active, Inactive, Maintenance)")]
    pub status: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CalculatePositionToolRequest {
    #[schemars(description = "Departure airport code")]
    pub departure: String,
    #[schemars(description = "Arrival airport code")]
    pub arrival: String,
    #[schemars(description = "Departure time (RFC3339)")]
    pub departure_time: String,
    #[schemars(description = "Arrival time (RFC3339)")]
    pub arrival_time: String,
    #[schemars(description = "Current time (RFC3339, optional)")]
    pub current_time: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct SatelliteTools {
    pub tool_router: ToolRouter<Self>,
}

#[tool_router]
impl SatelliteTools {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "List all satellites",
        annotations(read_only_hint = true, destructive_hint = false, idempotent_hint = true)
    )]
    pub async fn list_satellites(&self) -> Result<CallToolResult, McpError> {
        info!("Listing satellites");
        let satellites = fetch_satellites().await.map_err(|e| {
            error!("Failed to fetch satellites: {}", e);
            McpError::internal_error(
                "Failed to fetch satellites",
                Some(json!({"error": e.to_string()})),
            )
        })?;

        let mut result = String::new();
        for sat in &satellites {
            result.push_str(&format!(
                "{} (ID: {}) - Status: {:?}\n",
                sat.name, sat.id, sat.status
            ));
        }
        if result.is_empty() {
            result = "No satellites found.".to_string();
        }

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(
        description = "Create a new satellite",
        annotations(read_only_hint = false, destructive_hint = false, idempotent_hint = false)
    )]
    pub async fn create_satellite(
        &self,
        Parameters(CreateSatelliteToolRequest { name }): Parameters<CreateSatelliteToolRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("Creating satellite: {}", name);
        let req = CreateSatelliteRequest { name };
        let sat = create_satellite(req).await.map_err(|e| {
            error!("Failed to create satellite: {}", e);
            McpError::internal_error(
                "Failed to create satellite",
                Some(json!({"error": e.to_string()})),
            )
        })?;
        let result = format!(
            "Satellite created: {} (ID: {}) - Status: {:?}",
            sat.name, sat.id, sat.status
        );
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(
        description = "Update satellite status",
        annotations(read_only_hint = false, destructive_hint = true, idempotent_hint = true)
    )]
    pub async fn update_satellite_status(
        &self,
        Parameters(UpdateSatelliteStatusToolRequest { id, status }): Parameters<
            UpdateSatelliteStatusToolRequest,
        >,
    ) -> Result<CallToolResult, McpError> {
        info!("Updating satellite {} status to {}", id, status);
        let status_enum = match status.as_str() {
            "Active" => sky_tracer::model::SatelliteStatus::Active,
            "Inactive" => sky_tracer::model::SatelliteStatus::Inactive,
            "Maintenance" => sky_tracer::model::SatelliteStatus::Maintenance,
            _ => {
                return Err(McpError::invalid_params(
                    "Invalid status. Must be Active, Inactive, or Maintenance.",
                    None,
                ));
            }
        };
        let req = UpdateSatelliteStatusRequest {
            status: status_enum,
        };
        let sat = update_satellite_status(id, req).await.map_err(|e| {
            error!("Failed to update satellite status: {}", e);
            McpError::internal_error(
                "Failed to update satellite status",
                Some(json!({"error": e.to_string()})),
            )
        })?;
        let result = format!(
            "Satellite updated: {} (ID: {}) - Status: {:?}",
            sat.name, sat.id, sat.status
        );
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(
        description = "Calculate flight position using satellites",
        annotations(read_only_hint = true, destructive_hint = false, idempotent_hint = true)
    )]
    pub async fn calculate_position(
        &self,
        Parameters(req): Parameters<CalculatePositionToolRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!(
            "Calculating position for flight {} -> {}",
            req.departure, req.arrival
        );
        let request = CalculatePositionRequest {
            departure: req.departure,
            arrival: req.arrival,
            departure_time: chrono::DateTime::parse_from_rfc3339(&req.departure_time)
                .map_err(|e| {
                    McpError::invalid_params(
                        "Invalid departure_time",
                        Some(json!({"error": e.to_string()})),
                    )
                })?
                .with_timezone(&chrono::Utc),
            arrival_time: chrono::DateTime::parse_from_rfc3339(&req.arrival_time)
                .map_err(|e| {
                    McpError::invalid_params(
                        "Invalid arrival_time",
                        Some(json!({"error": e.to_string()})),
                    )
                })?
                .with_timezone(&chrono::Utc),
            current_time: req
                .current_time
                .map(|ct| {
                    chrono::DateTime::parse_from_rfc3339(&ct)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .map_err(|e| {
                            McpError::invalid_params(
                                "Invalid current_time",
                                Some(json!({"error": e.to_string()})),
                            )
                        })
                })
                .transpose()?,
        };
        let resp = calculate_position(request).await.map_err(|e| {
            error!("Failed to calculate position: {}", e);
            McpError::internal_error(
                "Failed to calculate position",
                Some(json!({"error": e.to_string()})),
            )
        })?;
        let mut result = String::new();
        for pos in &resp.positions {
            result.push_str(&format!(
                "Satellite: {}\nLat: {}\nLon: {}\nAlt: {}\n\n",
                pos.satellite_id, pos.latitude, pos.longitude, pos.altitude
            ));
        }
        if result.is_empty() {
            result = "No positions found.".to_string();
        }
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }
}

#[tool_handler]
impl ServerHandler for SatelliteTools {}
