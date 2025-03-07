use axum::{
    extract::{Query, State},
    response::Html,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sky_tracer::protocol::flights::FlightResponse;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use flight_controller::{
    flight_service::FlightService,
    openapi::ApiDoc,
    service::{create_flight, list_flights},
    ui::pages::{Home, HomeProps},
};

#[derive(Debug, Deserialize)]
struct PageParams {
    departure: Option<String>,
    arrival: Option<String>,
    date: Option<String>,
}

async fn render_page(
    Query(params): Query<PageParams>,
    State(flight_service): State<FlightService>,
) -> Html<String> {
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

#[tokio::main]
async fn main() {
    let flight_service = FlightService::new();

    let app = Router::new()
        .route("/", get(render_page))
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api/flights", post(create_flight))
        .route("/api/flights", get(list_flights))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(flight_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Flight Controller running on http://localhost:3001");
    println!("API documentation available at http://localhost:3001/api/docs");
    axum::serve(listener, app).await.unwrap();
}
