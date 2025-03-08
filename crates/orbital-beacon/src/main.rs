use axum::{
    routing::{get, post, put},
    Router,
};
use orbital_beacon::{openapi::ApiDoc, satellite_service::SatelliteService, service};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let satellite_service = SatelliteService::new();

    let app = Router::new()
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
