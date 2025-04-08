use axum::{
    extract::{Query, State},
    response::Html,
    routing::{get, post},
    Router,
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use chrono::{DateTime, Utc};
use flight_controller::{
    flight_service::FlightService,
    openapi::ApiDoc,
    service::{create_flight, get_flight_position, list_flights},
    ui::pages::{Home, HomeProps},
};
use serde::Deserialize;
use sky_tracer::protocol::flights::FlightResponse;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, instrument};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    let flight_service = FlightService::new();
    let api_doc = ApiDoc::openapi();

    let app = Router::new()
        .route("/", get(render_page))
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", api_doc.clone()))
        .merge(
            RapiDoc::with_openapi("/api-docs/rapidoc/openapi.json", api_doc.clone())
                .path("/api/rapidoc"),
        )
        .merge(Redoc::with_url("/api/redoc", api_doc))
        .route("/api/flights", post(create_flight))
        .route("/api/flights", get(list_flights))
        .route(
            "/api/flights/{flight_number}/position",
            get(get_flight_position),
        )
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(flight_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    info!("Server running on http://localhost:3001");
    info!("API documentation available at http://localhost:3001/api/docs");

    let server = axum::serve(listener, app);

    info!("Server started");

    // Run the server and handle shutdown
    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
