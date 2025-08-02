use crate::{routes, services::DelayService};
use axum::{routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use tower_http::cors::{Any, CorsLayer};

pub fn app(delay_service: DelayService) -> Router {
    Router::new()
        .route("/", get(routes::render_home_page))
        .route("/{airport_code}", get(routes::render_airport_delays))
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(delay_service)
}
