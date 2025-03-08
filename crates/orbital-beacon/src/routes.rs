use crate::satellite_service::SatelliteService;
use crate::service;
use crate::ui::pages::{Home, HomeProps, Launch, LaunchProps};
use axum::extract::{Query, State};
use axum::response::{Html, Redirect};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
pub struct FlashMessage {
    pub message: Option<String>,
}

#[axum::debug_handler]
pub async fn render_home(State(service): State<SatelliteService>) -> Html<String> {
    let satellites = service.list_satellites().await;
    let satellites = satellites
        .into_iter()
        .map(|s| sky_tracer::protocol::protocol::SatelliteResponse {
            id: s.id,
            name: s.name,
            status: s.status,
        })
        .collect();

    let renderer = yew::ServerRenderer::<Home>::with_props(move || HomeProps { satellites });

    let html = renderer.render().await;

    Html(format!(
        r#"<!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Orbital Beacon</title>
                <style>
                    {}
                </style>
            </head>
            <body>
                {}
            </body>
        </html>"#,
        include_str!("../assets/styles.css"),
        html
    ))
}

#[axum::debug_handler]
pub async fn render_launch(
    State(service): State<SatelliteService>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let flash_message = params.get("message").cloned();

    let renderer = yew::ServerRenderer::<Launch>::with_props(move || LaunchProps { flash_message });

    let html = renderer.render().await;

    Html(format!(
        r#"<!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Launch Satellite</title>
                <style>
                    {}
                </style>
            </head>
            <body>
                {}
            </body>
        </html>"#,
        include_str!("../assets/styles.css"),
        html
    ))
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
