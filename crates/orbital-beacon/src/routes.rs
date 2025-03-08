use crate::satellite_service::SatelliteService;
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
    let position_data =
        if let (Some(departure), Some(arrival), Some(departure_time), Some(arrival_time)) = (
            params.get("departure"),
            params.get("arrival"),
            params.get("departure_time"),
            params.get("arrival_time"),
        ) {
            // Parse the times
            let departure_time = DateTime::parse_from_rfc3339(departure_time)
                .map(|dt| dt.with_timezone(&Utc))
                .ok();
            let arrival_time = DateTime::parse_from_rfc3339(arrival_time)
                .map(|dt| dt.with_timezone(&Utc))
                .ok();

            if let (Some(departure_time), Some(arrival_time)) = (departure_time, arrival_time) {
                let request = sky_tracer::protocol::satellite::CalculatePositionRequest {
                    departure: departure.clone(),
                    arrival: arrival.clone(),
                    departure_time,
                    arrival_time,
                    current_time: None,
                };

                let (positions, departure_airport, arrival_airport) = service
                    .calculate_position(
                        &request.departure,
                        &request.arrival,
                        request.departure_time,
                        request.arrival_time,
                        request.current_time,
                    )
                    .await;

                let departure_airport_response = departure_airport
                    .map(|airport| sky_tracer::protocol::airports::AirportResponse::from(&airport));
                let arrival_airport_response = arrival_airport
                    .map(|airport| sky_tracer::protocol::airports::AirportResponse::from(&airport));

                Some(sky_tracer::protocol::satellite::CalculatePositionResponse {
                    positions,
                    departure_airport: departure_airport_response,
                    arrival_airport: arrival_airport_response,
                })
            } else {
                None
            }
        } else {
            None
        };

    let renderer =
        ServerRenderer::<FlightPosition>::with_props(move || FlightPositionProps { position_data });

    let body = renderer.render().await;
    render_html("Flight Position", body)
}
