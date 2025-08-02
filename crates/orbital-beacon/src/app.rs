use crate::{openapi, routes, services::SatelliteService};
use axum::Router;
use axum::routing::{get, post, put};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use sky_tracer::protocol::{
    SATELLITES_API_PATH, SATELLITES_POSITION_API_PATH, SATELLITES_STATUS_API_PATH,
};
use tower_http::cors::{Any, CorsLayer};

pub fn app(satellite_service: SatelliteService) -> Router {
    // API routes using protocol constants - these will be at /api/v1/...
    let api_routes = Router::new()
        .route(
            SATELLITES_API_PATH,
            post(routes::create_satellite).get(routes::list_satellites),
        )
        .route(
            SATELLITES_STATUS_API_PATH,
            put(routes::update_satellite_status),
        )
        .route(
            SATELLITES_POSITION_API_PATH,
            post(routes::calculate_position),
        );

    // Main application routes - these will be at / after prefix stripping
    Router::new()
        .route("/", get(routes::render_home)) // Home page at root after prefix strip
        .route(
            "/launch",
            get(routes::render_launch).post(routes::handle_launch),
        )
        .route("/update_status", get(routes::render_update_status))
        .route("/update_status/{id}", post(routes::handle_update_status))
        .route("/flight_position", get(routes::render_flight_position))
        .merge(openapi::routes()) // Clean openapi routes
        .merge(api_routes) // API routes are merged at their full paths
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .with_state(satellite_service)
}
