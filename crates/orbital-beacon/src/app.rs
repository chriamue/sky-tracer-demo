use crate::{openapi::ApiDoc, routes, services::SatelliteService, utils::get_path_prefix};
use axum::Router;
use axum::response::Redirect;
use axum::routing::{get, post, put};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use sky_tracer::protocol::{
    SATELLITES_API_PATH, SATELLITES_POSITION_API_PATH, SATELLITES_STATUS_API_PATH,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn app(satellite_service: SatelliteService) -> Router {
    let path_prefix = get_path_prefix();

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

    // Create redirect handler for /api/docs -> /prefix/docs
    let redirect_path = format!("{}/docs", path_prefix);
    let redirect_to_prefixed_docs = move || {
        let redirect_path = redirect_path.clone();
        async move { Redirect::permanent(&redirect_path) }
    };

    // Paths for SwaggerUI
    let docs_path = format!("{}/docs", path_prefix);
    let openapi_json_path = format!("{}/docs/openapi.json", path_prefix);

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
        // Redirect /api/docs to /satellites/docs
        .route("/api/docs", get(redirect_to_prefixed_docs))
        .merge(api_routes) // API routes are merged at their full paths
        // Serve docs at the prefixed path
        .merge(SwaggerUi::new(docs_path).url(openapi_json_path, ApiDoc::openapi()))
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
