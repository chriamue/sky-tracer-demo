use crate::satellite_service::SatelliteService;
use crate::service;
use crate::ui::pages::{Home, HomeProps, Launch, LaunchProps, UpdateStatus, UpdateStatusProps};
use axum::extract::{Path, Query, State};
use axum::response::{Html, Redirect};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Deserialize, Default)]
pub struct FlashMessage {
    pub message: Option<String>,
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

    let renderer = yew::ServerRenderer::<Home>::with_props(move || HomeProps { satellites });

    let body = renderer.render().await;
    render_html("Orbital Beacon", body)
}

#[axum::debug_handler]
pub async fn render_launch(
    State(_service): State<SatelliteService>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let flash_message = params.get("message").cloned();

    let renderer = yew::ServerRenderer::<Launch>::with_props(move || LaunchProps { flash_message });

    let body = renderer.render().await;
    render_html("Launch Satellite", body)
}

#[axum::debug_handler]
pub async fn handle_launch(
    State(service): State<SatelliteService>,
    axum::extract::Form(form): axum::extract::Form<service::CreateSatelliteForm>,
) -> Redirect {
    let satellite = service.create_satellite(form.name).await;
    let message = format!("Satellite '{}' launched successfully!", satellite.name);
    Redirect::to(&format!("/launch?message={}", message))
}

#[axum::debug_handler]
pub async fn render_update_status(
    State(_service): State<SatelliteService>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let flash_message = params.get("message").cloned();

    let renderer = yew::ServerRenderer::<UpdateStatus>::with_props(move || UpdateStatusProps {
        flash_message,
    });

    let body = renderer.render().await;
    render_html("Update Satellite Status", body)
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusForm {
    pub status: String,
}

#[axum::debug_handler]
pub async fn handle_update_status(
    State(service): State<SatelliteService>,
    Path(id): Path<Uuid>,
    axum::extract::Form(form): axum::extract::Form<UpdateStatusForm>,
) -> Redirect {
    use sky_tracer::model::SatelliteStatus;

    let status = match form.status.as_str() {
        "Active" => SatelliteStatus::Active,
        "Inactive" => SatelliteStatus::Inactive,
        "Maintenance" => SatelliteStatus::Maintenance,
        _ => {
            let message = format!("Invalid status: {}", form.status);
            return Redirect::to(&format!("/update_status?message={}", message));
        }
    };

    match service.update_status(id, status).await {
        Some(satellite) => {
            let message = format!(
                "Satellite '{}' status updated successfully!",
                satellite.name
            );
            Redirect::to(&format!("/update_status?message={}", message))
        }
        None => {
            let message = format!("Satellite with ID '{}' not found!", id);
            Redirect::to(&format!("/update_status?message={}", message))
        }
    }
}
