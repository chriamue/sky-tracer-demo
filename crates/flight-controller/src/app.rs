use crate::{
    openapi,
    routes::{create_flight, get_flight_position, list_flights},
    services::FlightService,
    ui::pages::{Home, HomeProps},
};
use axum::{
    extract::{Query, State},
    response::Html,
    routing::{get, post},
    Router,
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sky_tracer::protocol::flights::FlightResponse;
use sky_tracer::protocol::{FLIGHTS_API_PATH, FLIGHTS_POSITION_API_PATH};
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
struct PageParams {
    departure: Option<String>,
    arrival: Option<String>,
    date: Option<String>,
}

#[instrument(skip(params, flight_service))]
async fn render_page(
    Query(params): Query<PageParams>,
    State(flight_service): State<FlightService>,
) -> Html<String> {
    info!(?params, "Rendering page with search parameters");

    let date = params
        .date
        .and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let flights = flight_service
        .list_flights(params.departure, params.arrival, date)
        .await;

    let renderer = yew::ServerRenderer::<Home>::with_props(move || HomeProps {
        flights: flights
            .into_iter()
            .map(|f| FlightResponse {
                flight_number: f.flight_number,
                aircraft_number: f.aircraft_number,
                departure: f.departure,
                arrival: f.arrival,
                departure_time: f.departure_time,
                arrival_time: f.arrival_time,
            })
            .collect(),
    });

    let html = renderer.render().await;

    Html(format!(
        r#"<!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Flight Controller</title>
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

pub fn app() -> Router {
    let flight_service = FlightService::new();

    let api_router = Router::new()
        .route(FLIGHTS_API_PATH, post(create_flight).get(list_flights))
        .route(FLIGHTS_POSITION_API_PATH, get(get_flight_position))
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    Router::new()
        .route("/", get(render_page))
        .merge(openapi::routes())
        .merge(api_router)
        .with_state(flight_service)
}
