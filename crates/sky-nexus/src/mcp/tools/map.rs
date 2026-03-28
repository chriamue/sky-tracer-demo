use crate::services::airports::fetch_airports;
use crate::services::babel::{fetch_flight_position, fetch_flights_by_airport};
use crate::services::flights::fetch_flights;
use base64::Engine;
use flight_map::{AirportPin, RouteArc, rasterize, render_flight_map};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content, Implementation, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use tracing::{info, warn};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FlightMapRequest {
    #[schemars(
        description = "Optional airport IATA code to focus on (shows only its flights). Omit for the full network map."
    )]
    pub airport_code: Option<String>,
    #[schemars(description = "Optional map title")]
    pub title: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct MapTools {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl MapTools {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Generate an SVG flight map returned as a base64-encoded image.
    ///
    /// Only airports that participate in at least one route are shown.
    /// If a current position is known for a flight it is shown as a yellow dot.
    #[tool(
        description = "Generate a flight map image showing participating airports and active routes. Returns a base64-encoded SVG image."
    )]
    pub async fn generate_flight_map(
        &self,
        Parameters(req): Parameters<FlightMapRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("Generating flight map (focus: {:?})", req.airport_code);

        // ── Fetch airports ────────────────────────────────────────────────
        let all_airports = fetch_airports().await.map_err(|e| {
            McpError::internal_error(
                "Failed to fetch airports",
                Some(json!({"error": e.to_string()})),
            )
        })?;

        // ── Fetch flights ─────────────────────────────────────────────────
        let flights = if let Some(ref code) = req.airport_code {
            fetch_flights_by_airport(code).await.unwrap_or_default()
        } else {
            fetch_flights()
                .await
                .map_err(|e| {
                    McpError::internal_error(
                        "Failed to fetch flights",
                        Some(json!({"error": e.to_string()})),
                    )
                })?
                .into_iter()
                .map(|f| sky_tracer::protocol::flights::FlightResponse {
                    flight_number: f.flight_number,
                    aircraft_number: f.aircraft_number,
                    departure: f.departure,
                    arrival: f.arrival,
                    departure_time: f.departure_time,
                    arrival_time: f.arrival_time,
                })
                .collect()
        };

        // ── Airport lookup ────────────────────────────────────────────────
        let airport_map: HashMap<String, &sky_tracer::model::airport::Airport> =
            all_airports.iter().map(|a| (a.code.clone(), a)).collect();

        // ── Only include airports that appear in at least one route ────────
        let mut used_codes: HashSet<String> = HashSet::new();
        for f in &flights {
            used_codes.insert(f.departure.clone());
            used_codes.insert(f.arrival.clone());
        }

        let pins: Vec<AirportPin> = used_codes
            .iter()
            .filter_map(|code| {
                airport_map.get(code).map(|a| AirportPin {
                    code: a.code.clone(),
                    lat: a.latitude,
                    lon: a.longitude,
                })
            })
            .collect();

        // ── Build route arcs with optional live positions ─────────────────
        let mut arcs: Vec<RouteArc> = Vec::new();
        for flight in &flights {
            let dep = match airport_map.get(&flight.departure) {
                Some(a) => a,
                None => {
                    warn!("Unknown departure airport: {}", flight.departure);
                    continue;
                }
            };
            let arr = match airport_map.get(&flight.arrival) {
                Some(a) => a,
                None => {
                    warn!("Unknown arrival airport: {}", flight.arrival);
                    continue;
                }
            };

            let pos = fetch_flight_position(&flight.flight_number).await.ok();

            arcs.push(RouteArc {
                label: flight.flight_number.clone(),
                dep_lat: dep.latitude,
                dep_lon: dep.longitude,
                arr_lat: arr.latitude,
                arr_lon: arr.longitude,
                pos_lat: pos.as_ref().map(|p| p.latitude),
                pos_lon: pos.as_ref().map(|p| p.longitude),
            });
        }

        let title = req.title.or_else(|| {
            req.airport_code
                .as_ref()
                .map(|c| format!("Sky Tracer — {c}"))
                .or(Some("Sky Tracer Network".to_string()))
        });

        let svg = render_flight_map(pins, arcs, title);
        let png = rasterize(&svg).map_err(|e| {
            McpError::internal_error("Failed to rasterize map", Some(json!({"error": e})))
        })?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(&png);

        Ok(CallToolResult::success(vec![Content::image(
            encoded,
            "image/png",
        )]))
    }
}

#[tool_handler]
impl ServerHandler for MapTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "Flight map tool for Sky Nexus:\n\
                - generate_flight_map: Render a world map with airport pins and flight route arcs.\n\
                  Only airports participating in active routes are shown.\n\
                  Returns a base64-encoded SVG image (image/svg+xml)."
                    .to_string(),
            )
    }
}
