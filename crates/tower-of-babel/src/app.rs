use crate::{routes, services::BabelService};
use axum::{routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use sky_tracer::protocol::{BABEL_AIRPORT_API_PATH, BABEL_API_PATH, BABEL_POSITION_API_PATH};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn app(babel_service: BabelService) -> Router {
    let api_routes = Router::new()
        .route(
            // Direct use of the full path constant
            BABEL_AIRPORT_API_PATH,
            get(routes::get_flights_by_airport),
        )
        .route(
            // Direct use of the full path constant
            BABEL_POSITION_API_PATH,
            get(routes::get_flight_position),
        );

    Router::new()
        .merge(api_routes)
        .merge(
            SwaggerUi::new("/api/docs")
                .url("/api-docs/openapi.json", crate::openapi::ApiDoc::openapi()),
        )
        .merge(SwaggerUi::new(format!("{}/docs", BABEL_API_PATH)).url(
            format!("{}/api-docs/openapi.json", BABEL_API_PATH),
            crate::openapi::ApiDoc::openapi(),
        ))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(babel_service)
}
