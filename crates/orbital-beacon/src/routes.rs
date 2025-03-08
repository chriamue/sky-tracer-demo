use crate::satellite_service::{SatelliteService, SatelliteServiceError};
use crate::ui::pages::{
    FlightPosition, FlightPositionProps, Home, HomeProps, Launch, LaunchProps, UpdateStatus,
    UpdateStatusProps,
};
use crate::utils::get_path_prefix;
use axum::extract::{Path, Query, State};
use axum::response::{Html, Redirect};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;
use yew::ServerRenderer;

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
                <style>
                    {}
                </style>
            </head>
            <body>
                {}
            </body>
        </html>"#,
        include_str!("../assets/styles.css"),
        body
    ))
}

#[axum::debug_handler]
pub async fn render_home(State(service): State<SatelliteService>) -> Html<String> {
    let satellites = service.list_satellites().await;
    let satellites = satellites
        .into_iter()
        .map(|s| sky_tracer::protocol::satellite::SatelliteResponse {
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
pub async fn render_launch(
    State(_service): State<SatelliteService>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let flash_message = params.get("message").cloned();

    let renderer = ServerRenderer::<Launch>::with_props(move || LaunchProps { flash_message });

    let body = renderer.render().await;
    render_html("Launch Satellite", body)
}

#[axum::debug_handler]
pub async fn handle_launch(
    State(service): State<SatelliteService>,
    axum::extract::Form(form): axum::extract::Form<CreateSatelliteForm>,
) -> Redirect {
    let path_prefix = get_path_prefix();
    let satellite = service.create_satellite(form.name).await;
    let message = format!("Satellite '{}' launched successfully!", satellite.name);
    Redirect::to(&format!("{}/launch?message={}", path_prefix, message))
}

#[axum::debug_handler]
pub async fn render_update_status(
    State(_service): State<SatelliteService>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let flash_message = params.get("message").cloned();

    let renderer =
        ServerRenderer::<UpdateStatus>::with_props(move || UpdateStatusProps { flash_message });

    let body = renderer.render().await;
    render_html("Update Satellite Status", body)
}

#[axum::debug_handler]
pub async fn handle_update_status(
    State(service): State<SatelliteService>,
    Path(id): Path<Uuid>,
    axum::extract::Form(form): axum::extract::Form<UpdateStatusForm>,
) -> Redirect {
    use sky_tracer::model::SatelliteStatus;
    let path_prefix = get_path_prefix();

    let status = match form.status.as_str() {
        "Active" => SatelliteStatus::Active,
        "Inactive" => SatelliteStatus::Inactive,
        "Maintenance" => SatelliteStatus::Maintenance,
        _ => {
            let message = format!("Invalid status: {}", form.status);
            return Redirect::to(&format!(
                "{}/update_status?message={}",
                path_prefix, message
            ));
        }
    };

    match service.update_status(id, status).await {
        Some(satellite) => {
            let message = format!(
                "Satellite '{}' status updated successfully!",
                satellite.name
            );
            Redirect::to(&format!(
                "{}/update_status?message={}",
                path_prefix, message
            ))
        }
        None => {
            let message = format!("Satellite with ID '{}' not found!", id);
            Redirect::to(&format!(
                "{}/update_status?message={}",
                path_prefix, message
            ))
        }
    }
}

#[axum::debug_handler]
pub async fn render_flight_position(
    State(service): State<SatelliteService>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let (position_data, error_message) =
        if let (Some(departure), Some(arrival), Some(departure_time), Some(arrival_time)) = (
            params.get("departure"),
            params.get("arrival"),
            params.get("departure_time"),
            params.get("arrival_time"),
        ) {
            let parse_datetime = |dt_str: &str| -> Option<DateTime<Utc>> {
                if let Ok(dt) = DateTime::parse_from_str(
                    &format!("{}:00+00:00", dt_str),
                    "%Y-%m-%dT%H:%M:%S%:z",
                ) {
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
                    match service
                        .calculate_position(departure, arrival, departure_time, arrival_time, None)
                        .await
                    {
                        Ok((positions, departure_airport, arrival_airport)) => {
                            if positions.is_empty() {
                                (
                                    None,
                                    Some("Flight is not currently in progress.".to_string()),
                                )
                            } else {
                                let departure_airport_response = departure_airport.map(|airport| {
                                    sky_tracer::protocol::airports::AirportResponse::from(&airport)
                                });
                                let arrival_airport_response = arrival_airport.map(|airport| {
                                    sky_tracer::protocol::airports::AirportResponse::from(&airport)
                                });

                                (
                                Some(sky_tracer::protocol::satellite::CalculatePositionResponse {
                                    positions,
                                    departure_airport: departure_airport_response,
                                    arrival_airport: arrival_airport_response,
                                }),
                                None,
                            )
                            }
                        }
                        Err(SatelliteServiceError::NoActiveSatellites) => (
                            None,
                            Some("No active satellites available for tracking.".to_string()),
                        ),
                        Err(SatelliteServiceError::AirportNotFound(code)) => {
                            (None, Some(format!("Airport not found: {}", code)))
                        }
                        Err(SatelliteServiceError::AirportFetchError(e)) => {
                            (None, Some(format!("Failed to fetch airport data: {}", e)))
                        }
                    }
                }
                _ => (
                    None,
                    Some(
                        "Invalid date/time format. Please use YYYY-MM-DDTHH:MM format.".to_string(),
                    ),
                ),
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
