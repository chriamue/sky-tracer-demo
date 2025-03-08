use axum::routing::{get, post, put};
use axum::Router;
use orbital_beacon::openapi::ApiDoc;
use orbital_beacon::satellite_service::SatelliteService;
use orbital_beacon::service;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use orbital_beacon::routes;

#[tokio::main]
async fn main() {
    let satellite_service = SatelliteService::new();

    let app = Router::new()
        .route("/", get(routes::render_home))
        .route(
            "/launch",
            get(routes::render_launch).post(routes::handle_launch),
        )
        .route("/api/satellites", post(service::create_satellite))
        .route(
            "/api/satellites/{id}/status",
            put(service::update_satellite_status),
        )
        .route("/api/satellites", get(service::list_satellites))
        .route("/api/position", post(service::calculate_position))
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(satellite_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    println!("Orbital Beacon running on http://localhost:3002");
    println!("API documentation available at http://localhost:3002/api/docs");
    axum::serve(listener, app).await.unwrap();
}
