pub mod models;
pub mod openapi;
pub mod routes;
pub mod services;

#[cfg(feature = "ssr")]
pub mod ui;

use axum::{routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use sky_tracer::protocol::{AIRPORTS_API_PATH, AIRPORTS_SEARCH_API_PATH};
use tower_http::cors::{Any, CorsLayer};

pub fn app() -> Router {
    let api_router = Router::new()
        .route(AIRPORTS_API_PATH, get(routes::list_airports))
        .route(AIRPORTS_SEARCH_API_PATH, get(routes::search_airports))
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    Router::new().merge(openapi::routes()).merge(api_router)
}
