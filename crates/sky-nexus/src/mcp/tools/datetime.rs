use crate::models::datetime::*;
use crate::services::datetime::{
    DateTimeServiceError, compare_timezones, get_aviation_times, get_current_datetime,
};
use chrono::{DateTime, Offset, Utc};
use chrono_tz::{OffsetName, Tz};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetCurrentDateTimeRequest {
    #[schemars(
        description = "Timezone (optional, defaults to UTC). Examples: UTC, America/New_York, Europe/London"
    )]
    pub timezone: Option<String>,
    #[schemars(
        description = "Format string (optional, defaults to RFC3339). Examples: %Y-%m-%d %H:%M:%S, %Y-%m-%d"
    )]
    pub format: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TimezoneDifferenceRequest {
    #[schemars(description = "Source timezone (e.g., America/New_York)")]
    pub from_timezone: String,
    #[schemars(description = "Target timezone (e.g., Europe/London)")]
    pub to_timezone: String,
}

#[derive(Clone, Debug, Default)]
pub struct DateTimeTools {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl DateTimeTools {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get the current date and time")]
    pub async fn get_current_datetime(
        &self,
        Parameters(req): Parameters<GetCurrentDateTimeRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!(
            "Getting current datetime with timezone: {:?}, format: {:?}",
            req.timezone, req.format
        );

        let query = GetDateTimeQuery {
            timezone: req.timezone,
            format: req.format,
        };

        match get_current_datetime(query).await {
            Ok(response) => {
                let result = format!(
                    "Current date and time: {}\n\
                     Timezone: {}\n\
                     Unix timestamp: {}\n\
                     ISO week: {}\n\
                     Day of year: {}\n\
                     UTC offset: {:+} seconds",
                    response.formatted_time,
                    response.timezone,
                    response.unix_timestamp,
                    response.iso_week,
                    response.day_of_year,
                    response.utc_offset
                );

                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(DateTimeServiceError::InvalidTimezone(msg)) => {
                error!("Invalid timezone: {}", msg);
                Err(McpError::invalid_params(
                    "Invalid timezone",
                    Some(json!({"error": msg})),
                ))
            }
            Err(e) => {
                error!("Failed to get current datetime: {}", e);
                Err(McpError::internal_error(
                    "Failed to get current datetime",
                    Some(json!({"error": e.to_string()})),
                ))
            }
        }
    }

    #[tool(description = "Get current time in multiple aviation-relevant timezones")]
    pub async fn get_aviation_times(&self) -> Result<CallToolResult, McpError> {
        info!("Getting current time in aviation-relevant timezones");

        match get_aviation_times().await {
            Ok(response) => {
                let mut result = String::from("Current time in major aviation hubs:\n\n");

                for time_info in &response.times {
                    result.push_str(&format!(
                        "{}: {} ({}, UTC{:+})\n",
                        time_info.name,
                        time_info.local_time,
                        time_info.abbreviation,
                        time_info.utc_offset_hours
                    ));
                }

                result.push_str(&format!(
                    "\nUTC Unix timestamp: {}",
                    response.unix_timestamp
                ));

                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                error!("Failed to get aviation times: {}", e);
                Err(McpError::internal_error(
                    "Failed to get aviation times",
                    Some(json!({"error": e.to_string()})),
                ))
            }
        }
    }

    /// Compare two timezones and return their time difference as structured content
    #[tool(
        name = "compare_timezones",
        description = "Compare two timezones and show the time difference"
    )]
    pub async fn compare_timezones(
        &self,
        params: Parameters<TimezoneComparisonRequest>,
    ) -> Result<CallToolResult, McpError> {
        let req = &params.0;

        // Parse timezones
        let from_tz: Tz = req.from_timezone.parse().map_err(|_| {
            McpError::invalid_params(
                "Invalid source timezone",
                Some(json!({"error": "Invalid source timezone"})),
            )
        })?;
        let to_tz: Tz = req.to_timezone.parse().map_err(|_| {
            McpError::invalid_params(
                "Invalid target timezone",
                Some(json!({"error": "Invalid target timezone"})),
            )
        })?;

        // Get current UTC time
        let now: DateTime<Utc> = Utc::now();

        // Convert to local times
        let from_local = now.with_timezone(&from_tz);
        let to_local = now.with_timezone(&to_tz);

        // Calculate offsets using the correct method
        let from_offset_seconds = from_local.offset().fix().local_minus_utc();
        let to_offset_seconds = to_local.offset().fix().local_minus_utc();

        let from_offset = from_offset_seconds as f32 / 3600.0;
        let to_offset = to_offset_seconds as f32 / 3600.0;

        let diff_hours = to_offset - from_offset;

        // Format description
        let description = if diff_hours > 0.0 {
            format!(
                "{} is {} hours ahead of {}",
                req.to_timezone, diff_hours, req.from_timezone
            )
        } else if diff_hours < 0.0 {
            format!(
                "{} is {} hours behind of {}",
                req.to_timezone, -diff_hours, req.from_timezone
            )
        } else {
            format!("Both timezones are in sync")
        };

        // Create response object
        let resp = TimezoneComparisonResponse {
            from: TimezoneInfo {
                timezone: req.from_timezone.clone(),
                local_time: from_local.to_rfc3339(),
                utc_offset_hours: from_offset,
                abbreviation: from_local
                    .offset()
                    .abbreviation()
                    .unwrap_or("UTC")
                    .to_string(),
            },
            to: TimezoneInfo {
                timezone: req.to_timezone.clone(),
                local_time: to_local.to_rfc3339(),
                utc_offset_hours: to_offset,
                abbreviation: to_local
                    .offset()
                    .abbreviation()
                    .unwrap_or("UTC")
                    .to_string(),
            },
            difference_hours: diff_hours,
            description,
        };

        // Convert to JSON for structured content
        let json_result = serde_json::to_value(&resp).map_err(|e| {
            error!("Failed to serialize timezone comparison: {}", e);
            McpError::internal_error(
                "Failed to process timezone comparison",
                Some(json!({"error": e.to_string()})),
            )
        })?;

        // Return as structured content in CallToolResult
        Ok(CallToolResult::structured(json_result))
    }

    #[tool(description = "Calculate time difference between two timezones")]
    pub async fn get_timezone_difference(
        &self,
        Parameters(req): Parameters<TimezoneDifferenceRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!(
            "Calculating timezone difference between {} and {}",
            req.from_timezone, req.to_timezone
        );

        let comparison_request = TimezoneComparisonRequest {
            from_timezone: req.from_timezone,
            to_timezone: req.to_timezone,
        };

        match compare_timezones(comparison_request).await {
            Ok(response) => {
                let result = format!(
                    "Timezone Comparison:\n\
                     From: {} - {} (UTC{:+}, {})\n\
                     To: {} - {} (UTC{:+}, {})\n\
                     \n\
                     {}",
                    response.from.timezone,
                    response.from.local_time,
                    response.from.utc_offset_hours,
                    response.from.abbreviation,
                    response.to.timezone,
                    response.to.local_time,
                    response.to.utc_offset_hours,
                    response.to.abbreviation,
                    response.description
                );

                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(DateTimeServiceError::InvalidTimezone(msg)) => {
                error!("Invalid timezone: {}", msg);
                Err(McpError::invalid_params(
                    "Invalid timezone",
                    Some(json!({"error": msg})),
                ))
            }
            Err(e) => {
                error!("Failed to calculate timezone difference: {}", e);
                Err(McpError::internal_error(
                    "Failed to calculate timezone difference",
                    Some(json!({"error": e.to_string()})),
                ))
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for DateTimeTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "sky-nexus-mcp-datetime".to_string(),
                version: "0.1.0".to_string(),
            },
            instructions: Some(
                "DateTime tools for Sky Nexus:\n\
                - get_current_datetime: Get current date and time with optional timezone and formatting\n\
                - get_aviation_times: Get current time in major aviation hubs around the world\n\
                - get_timezone_difference: Calculate time difference between two timezones\n\
                \n\
                Useful for flight scheduling, coordination across time zones, and aviation operations."
                    .to_string(),
            ),
        }
    }
}
