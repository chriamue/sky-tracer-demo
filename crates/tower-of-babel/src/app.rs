use crate::{openapi, routes, services::BabelService};
use axum::{routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use sky_tracer::protocol::{BABEL_AIRPORT_API_PATH, BABEL_POSITION_API_PATH};
use tower_http::cors::{Any, CorsLayer};

pub fn app(babel_service: BabelService) -> Router {
    let api_routes = Router::new()
        .route(BABEL_AIRPORT_API_PATH, get(routes::get_flights_by_airport))
        .route(BABEL_POSITION_API_PATH, get(routes::get_flight_position));

    Router::new()
        .merge(openapi::routes()) // Now works with generic state
        .merge(api_routes) // This has the BabelService state
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(babel_service) // Apply state at the end
}
