use axum::routing::{get, post, put};
use axum::Router;
use orbital_beacon::openapi::ApiDoc;
use orbital_beacon::routes;
use orbital_beacon::satellite_service::SatelliteService;
use orbital_beacon::service;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

fn get_path_prefix() -> String {
    std::env::var("PATH_PREFIX").unwrap_or_else(|_| "".to_string())
}

#[tokio::main]
async fn main() {
    let airport_service_url =
        env::var("AIRPORTS_SERVICE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let satellite_service = SatelliteService::new(airport_service_url);

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
        .with_state(satellite_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    let path_prefix = get_path_prefix();
    println!(
        "Orbital Beacon running on http://localhost:3002 (external: {})",
        path_prefix
    );
    println!(
        "API documentation available at http://localhost:3002/api/docs (external: {}/api/docs)",
        path_prefix
    );
    axum::serve(listener, app).await.unwrap();
}
