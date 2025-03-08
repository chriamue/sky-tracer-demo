use axum::routing::{get, post, put};
use axum::Router;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

use orbital_beacon::openapi::ApiDoc;
use orbital_beacon::routes;
use orbital_beacon::satellite_service::SatelliteService;
use orbital_beacon::service;
use orbital_beacon::utils::get_path_prefix;
use sky_tracer::prelude::*;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::info;
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // Initialize telemetry
    //setup_telemetry().expect("Failed to setup telemetry");

    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers().unwrap();

    let airport_service_url =
        env::var("AIRPORTS_SERVICE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    info!("Starting Orbital Beacon service");
    info!(airport_service_url = %airport_service_url, "Configured airport service");

    let satellite_service = SatelliteService::new(airport_service_url);

    // Configure trace layer
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR));

    let api_routes = Router::new()
        .route("/satellites", post(service::create_satellite))
        .route(
            "/satellites/{id}/status",
            put(service::update_satellite_status),
        )
        .route("/satellites", get(service::list_satellites))
        .route("/position", post(service::calculate_position));

    let app = Router::new()
        .route("/", get(routes::render_home))
        .route(
            "/launch",
            get(routes::render_launch).post(routes::handle_launch),
        )
        .route("/update_status", get(routes::render_update_status))
        .route("/update_status/{id}", post(routes::handle_update_status))
        .route("/flight_position", get(routes::render_flight_position))
        .nest("/api", api_routes)
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(trace_layer)
        .with_state(satellite_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    let path_prefix = get_path_prefix();
    info!(
        path_prefix = %path_prefix,
        "Server starting"
    );

    info!("API documentation available at http://localhost:3002/api/docs");

    axum::serve(listener, app).await.unwrap();
}
