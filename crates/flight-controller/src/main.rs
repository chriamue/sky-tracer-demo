use crate::flight_service::FlightService;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use flight_controller::flight_service;
use flight_controller::openapi::ApiDoc;
use flight_controller::service::create_flight;
use flight_controller::service::list_flights;

#[tokio::main]
async fn main() {
    let flight_service = FlightService::new();

    let app = Router::new()
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api/flights", post(create_flight))
        .route("/api/flights", get(list_flights))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(flight_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Flight Controller running on http://localhost:3001");
    println!("API documentation available at http://localhost:3001/swagger-ui");
    axum::serve(listener, app).await.unwrap();
}
