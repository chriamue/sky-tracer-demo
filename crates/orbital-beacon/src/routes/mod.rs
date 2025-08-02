pub mod api;

use crate::models::FlightPositionRequest;
use crate::services::{SatelliteService, SatelliteServiceError};
use crate::ui::pages::{
    FlightPosition, FlightPositionProps, Home, HomeProps, Launch, LaunchProps, UpdateStatus,
    UpdateStatusProps,
};
use crate::utils::get_path_prefix;
use axum::extract::{Path, Query, State};
use axum::response::{Html, Redirect};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sky_tracer::model::SatelliteStatus;
use sky_tracer::protocol::satellite::SatelliteResponse;
use std::collections::HashMap;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;
use yew::ServerRenderer;

// Re-export API handlers for convenience
pub use api::{calculate_position, create_satellite, list_satellites, update_satellite_status};

#[derive(Debug, Deserialize, Default)]
pub struct FlashMessage {
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSatelliteForm {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusForm {
    pub status: String,
}

fn render_html(title: &str, body: String) -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>{title}</title>
                <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
                <style>
                    {}
                    {}
                </style>
            </head>
            <body>
                {body}
                <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
                <script>{}</script>
            </body>
        </html>"#,
        include_str!("../../assets/styles.css"),
        include_str!("../../assets/orbital.css"),
        include_str!("../../assets/map.js"),
    ))
}

#[axum::debug_handler]
#[instrument(skip(service))]
pub async fn render_home(State(service): State<SatelliteService>) -> Html<String> {
    info!("Rendering home page");

    let satellites = service.list_satellites().await;
    let satellites = satellites
        .into_iter()
        .map(|s| SatelliteResponse {
            id: s.id,
            name: s.name,
            status: s.status,
        })
        .collect();

    let renderer = ServerRenderer::<Home>::with_props(move || HomeProps { satellites });

    let body = renderer.render().await;
    render_html("Orbital Beacon", body)
}

#[axum::debug_handler]
#[instrument(skip(_service))]
pub async fn render_launch(
    State(_service): State<SatelliteService>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    info!("Rendering launch page");
    let flash_message = params.get("message").cloned();

    let renderer = ServerRenderer::<Launch>::with_props(move || LaunchProps { flash_message });

    let body = renderer.render().await;
    render_html("Launch Satellite", body)
}

#[axum::debug_handler]
#[instrument(skip(service))]
pub async fn handle_launch(
    State(service): State<SatelliteService>,
    axum::extract::Form(form): axum::extract::Form<CreateSatelliteForm>,
) -> Redirect {
    info!(satellite_name = %form.name, "Handling satellite launch request");

    let path_prefix = get_path_prefix();

    match service.create_satellite(form.name.clone()).await {
        Ok(satellite) => {
            let message = format!("Satellite '{}' launched successfully!", satellite.name);
            info!(satellite_id = %satellite.id, message = %message, "Satellite launched successfully");
            Redirect::to(&format!("{}/launch?message={}", path_prefix, message))
        }
        Err(e) => {
            error!(error = %e, satellite_name = %form.name, "Failed to launch satellite");
            let message = format!("Failed to launch satellite: {}", e);
            Redirect::to(&format!("{}/launch?message={}", path_prefix, message))
        }
    }
}

#[axum::debug_handler]
#[instrument(skip(_service))]
pub async fn render_update_status(
    State(_service): State<SatelliteService>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    info!("Rendering update status page");
    let flash_message = params.get("message").cloned();

    let renderer =
        ServerRenderer::<UpdateStatus>::with_props(move || UpdateStatusProps { flash_message });

    let body = renderer.render().await;
    render_html("Update Satellite Status", body)
}

#[axum::debug_handler]
#[instrument(skip(service))]
pub async fn handle_update_status(
    State(service): State<SatelliteService>,
    Path(id): Path<Uuid>,
    axum::extract::Form(form): axum::extract::Form<UpdateStatusForm>,
) -> Redirect {
    info!(satellite_id = %id, new_status = %form.status, "Handling satellite status update");

    let path_prefix = get_path_prefix();

    let status = match form.status.as_str() {
        "Active" => SatelliteStatus::Active,
        "Inactive" => SatelliteStatus::Inactive,
        "Maintenance" => SatelliteStatus::Maintenance,
        _ => {
            warn!(invalid_status = %form.status, "Invalid status provided");
            let message = format!("Invalid status: {}", form.status);
            return Redirect::to(&format!(
                "{}/update_status?message={}",
                path_prefix, message
            ));
        }
    };

    match service.update_satellite_status(id, status).await {
        Ok(satellite) => {
            let message = format!(
                "Satellite '{}' status updated successfully!",
                satellite.name
            );
            info!(
                satellite_id = %id,
                satellite_name = %satellite.name,
                new_status = ?status,
                "Satellite status updated successfully"
            );
            Redirect::to(&format!(
                "{}/update_status?message={}",
                path_prefix, message
            ))
        }
        Err(SatelliteServiceError::InvalidSatelliteId(_)) => {
            warn!(satellite_id = %id, "Satellite not found for status update");
            let message = format!("Satellite with ID '{}' not found!", id);
            Redirect::to(&format!(
                "{}/update_status?message={}",
                path_prefix, message
            ))
        }
        Err(e) => {
            error!(satellite_id = %id, error = %e, "Failed to update satellite status");
            let message = format!("Failed to update satellite status: {}", e);
            Redirect::to(&format!(
                "{}/update_status?message={}",
                path_prefix, message
            ))
        }
    }
}

#[axum::debug_handler]
#[instrument(skip(service))]
pub async fn render_flight_position(
    State(service): State<SatelliteService>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    info!("Rendering flight position page");

    let (position_data, error_message) = if let (
        Some(departure),
        Some(arrival),
        Some(departure_time),
        Some(arrival_time),
    ) = (
        params.get("departure"),
        params.get("arrival"),
        params.get("departure_time"),
        params.get("arrival_time"),
    ) {
        info!(
            departure = %departure,
            arrival = %arrival,
            departure_time = %departure_time,
            arrival_time = %arrival_time,
            "Processing flight position request"
        );

        let parse_datetime = |dt_str: &str| -> Option<DateTime<Utc>> {
            if let Ok(dt) =
                DateTime::parse_from_str(&format!("{}:00+00:00", dt_str), "%Y-%m-%dT%H:%M:%S%:z")
            {
                return Some(dt.with_timezone(&Utc));
            }

            if let Ok(dt) =
                DateTime::parse_from_str(&format!("{}+00:00", dt_str), "%Y-%m-%dT%H:%M%:z")
            {
                return Some(dt.with_timezone(&Utc));
            }

            None
        };

        let departure_time = parse_datetime(departure_time);
        let arrival_time = parse_datetime(arrival_time);

        match (departure_time, arrival_time) {
            (Some(departure_time), Some(arrival_time)) => {
                let request = FlightPositionRequest::new(
                    departure.clone(),
                    arrival.clone(),
                    departure_time,
                    arrival_time,
                );

                match service.calculate_flight_position(request).await {
                    Ok(calculation) => {
                        if calculation.positions.is_empty() {
                            info!("Flight is not currently in progress");
                            (
                                None,
                                Some("Flight is not currently in progress.".to_string()),
                            )
                        } else {
                            let departure_airport_response =
                                calculation.departure_airport.map(|airport| {
                                    sky_tracer::protocol::airports::AirportResponse::from(&airport)
                                });
                            let arrival_airport_response =
                                calculation.arrival_airport.map(|airport| {
                                    sky_tracer::protocol::airports::AirportResponse::from(&airport)
                                });

                            info!(
                                positions_count = calculation.positions.len(),
                                "Successfully calculated flight position"
                            );

                            (
                                Some(sky_tracer::protocol::satellite::CalculatePositionResponse {
                                    positions: calculation.positions,
                                    departure_airport: departure_airport_response,
                                    arrival_airport: arrival_airport_response,
                                }),
                                None,
                            )
                        }
                    }
                    Err(SatelliteServiceError::NoActiveSatellites) => {
                        warn!("No active satellites available for tracking");
                        (
                            None,
                            Some("No active satellites available for tracking.".to_string()),
                        )
                    }
                    Err(SatelliteServiceError::AirportNotFound(code)) => {
                        warn!(airport_code = %code, "Airport not found");
                        (None, Some(format!("Airport not found: {}", code)))
                    }
                    Err(SatelliteServiceError::AirportFetchError(e)) => {
                        error!(error = %e, "Failed to fetch airport data");
                        (None, Some(format!("Failed to fetch airport data: {}", e)))
                    }
                    Err(e) => {
                        error!(error = %e, "Failed to calculate flight position");
                        (None, Some(format!("Failed to calculate position: {}", e)))
                    }
                }
            }
            _ => {
                warn!("Invalid date/time format provided");
                (
                    None,
                    Some(
                        "Invalid date/time format. Please use YYYY-MM-DDTHH:MM format.".to_string(),
                    ),
                )
            }
        }
    } else {
        (None, None)
    };

    let renderer = ServerRenderer::<FlightPosition>::with_props(move || FlightPositionProps {
        position_data,
        error_message,
    });

    let body = renderer.render().await;
    render_html("Flight Position", body)
}
