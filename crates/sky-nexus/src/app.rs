use crate::{openapi, routes};
use axum::Router;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

pub fn app() -> Router {
    Router::new()
        .merge(openapi::routes())
        .merge(routes::create_router())
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
}
